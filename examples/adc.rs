#![no_std]
#![no_main]

use bl702_hal::{
    adc::*,
    clock::{board_clock_init, system_init, ClockConfig},
    pac,
    prelude::*,
    uart::*,
};
use core::fmt::Write;
use core::write;
use embedded_hal_alpha::delay::blocking::DelayMs;

use embedded_hal_alpha::digital::blocking::OutputPin;
#[cfg(not(feature = "panic_serial"))]
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    // This *MUST* be called first
    system_init();
    // Set up default board clock config
    board_clock_init();

    let mut d = bl702_hal::delay::McycleDelay::new(bl702_hal::clock::system_frequency());
    d.delay_ms(1000).unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let mut led = parts.pin24.into_pull_up_output();
    led.set_low().unwrap();
    led.set_high().unwrap();

    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    // Set up uart output. Since this microcontroller has a pin matrix,
    // we need to set up both the pins and the muxs
    let tx = parts.pin14.into_uart_sig6();
    let rx = parts.pin15.into_uart_sig7();
    let mux6 = parts.uart_mux6.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();
    // Configure our UART to 2MBaud, and use the pins we configured above
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(115200.Bd()),
        ((tx, mux6), (rx, mux7)),
        clocks,
    );

    let mut adc = Adc::adc(
        dp.AON,
        config::AdcConfig::default()
            .resolution(config::Resolution::Sixteen15k565)
            .clock(config::Clock::Div32),
    );
    let ain = parts.pin9.into_analog();

    // Create a blocking delay function based on the current cpu frequency
    let mut d = bl702_hal::delay::McycleDelay::new(bl702_hal::SYSFREQ);

    //let hello = "bl702 adc={}\r\n";
    let mut count = 0;
    loop {
        d.delay_ms(250).unwrap();
        let sample = adc.convert(&ain);
        let mv = adc.sample_to_millivolts(sample);
        let t = write!(serial, "bl702 count={} adc={}\r\n", count, mv);
        //let t = serial.write_str(hello);
        let _ = match t {
            Ok(_) => led.set_high().unwrap(),
            Err(_) => led.set_low().unwrap(),
        };
        count = count + 1;
    }
}
