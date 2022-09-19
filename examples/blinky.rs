#![no_std]
#![no_main]

use bl702_hal as hal;
use embedded_hal::digital::blocking::OutputPin;
use hal::{delay::McycleDelay, pac, prelude::*};
use panic_halt as _;

use embedded_hal::delay::blocking::DelayMs;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let parts = dp.GLB.split();

    let mut gpio2 = parts.pin2.into_pull_up_output();

    // Create a blocking delay function based on the current cpu frequency
    let mut d = McycleDelay::new(bl702_hal::SYSFREQ);

    loop {
        gpio2.set_high().unwrap();
        d.delay_ms(1000).unwrap();

        gpio2.set_low().unwrap();
        d.delay_ms(1000).unwrap();
    }
}
