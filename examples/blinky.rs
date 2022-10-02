#![no_std]
#![no_main]

use bl702_hal as hal;
use embedded_hal::digital::blocking::OutputPin;
use hal::{
    clock::{board_clock_init, system_init, ClockConfig},
    delay::McycleDelay,
    pac,
    prelude::*,
};
use panic_halt as _;

use embedded_hal::delay::blocking::DelayMs;

#[riscv_rt::entry]
fn main() -> ! {
    // This *MUST* be called first
    system_init();
    // Set up default board clock config
    board_clock_init();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    let mut gpio17 = parts.pin17.into_pull_up_output();

    // Create a blocking delay function based on the current cpu frequency
    let mut d = McycleDelay::new(clocks.sysclk().0);

    loop {
        gpio17.set_high().unwrap();
        d.delay_ms(1000).unwrap();

        gpio17.set_low().unwrap();
        d.delay_ms(1000).unwrap();
    }
}
