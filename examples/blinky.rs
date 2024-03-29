#![no_std]
#![no_main]

use bl702_hal as hal;
use embedded_hal_alpha::digital::blocking::OutputPin;
use hal::{
    clock::{board_clock_init, system_init, ClockConfig},
    delay::McycleDelay,
    pac,
    prelude::*,
};
#[cfg(not(feature = "panic_serial"))]
use panic_halt as _;

use embedded_hal_alpha::delay::blocking::DelayMs;

#[riscv_rt::entry]
fn main() -> ! {
    // This *MUST* be called first
    system_init();
    // Set up default board clock config
    board_clock_init();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    let mut led = parts.pin17.into_pull_up_output();

    // Create a blocking delay function based on the current cpu frequency
    let mut d = McycleDelay::new(clocks.sysclk().0);

    loop {
        led.set_high().unwrap();
        d.delay_ms(1000).unwrap();

        led.set_low().unwrap();
        d.delay_ms(1000).unwrap();
    }
}
