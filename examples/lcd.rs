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

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use st7735_lcd;
use st7735_lcd::Orientation;

#[riscv_rt::entry]
fn main() -> ! {
    // This *MUST* be called first
    system_init();
    // Set up default board clock config
    board_clock_init();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    let sclk = parts.pin23.into_spi_sclk();
    let mosi = parts.pin24.into_spi_mosi();
    let miso = parts.pin29.into_spi_miso(); // unbonded on bl702
    let dc = parts.pin25.into_floating_output();
    let mut cs = parts.pin1.into_floating_output();
    let rst = parts.pin10.into_floating_output(); // unbonded on bl702

    cs.set_high().unwrap();

    let spi = hal::spi::Spi::new(
        dp.SPI,
        (miso, mosi, sclk),
        embedded_hal_alpha::spi::MODE_0,
        9_000_000u32.Hz(), // fastest that obeys st7735 minimum high/low time with 36mhz bclk
        clocks,
    );

    let mut d = McycleDelay::new(clocks.sysclk().0);
    cs.set_low().unwrap();

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, true, 160, 80);
    disp.init(&mut d).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.set_offset(0, 25);
    disp.clear(Rgb565::BLACK).unwrap();

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let image: Image<_, Rgb565> = Image::new(&image_raw, Point::new(34, 8));
    image.draw(&mut disp).unwrap();

    loop {}
}
