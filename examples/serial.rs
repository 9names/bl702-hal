#![no_std]
#![no_main]

use bl702_hal::{
    clock::{board_clock_init, system_init, ClockConfig},
    pac,
    prelude::*,
    uart::*,
};
use core::fmt::Write;
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
    let mut led = parts.pin17.into_pull_up_output();
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
        Config::default().baudrate(2_000_000.Bd()),
        ((tx, mux6), (rx, mux7)),
        clocks,
    );

    // Create a blocking delay function based on the current cpu frequency
    let mut d = bl702_hal::delay::McycleDelay::new(bl702_hal::SYSFREQ);

    let hello = "hello rust!\r\n";
    loop {
        d.delay_ms(100).unwrap();
        let t = serial.write_str(hello);
        let _ = match t {
            Ok(_) => led.set_high().unwrap(),
            Err(_) => led.set_low().unwrap(),
        };
    }
}
