//! # HAL for the BL702 microcontroller
//!
//! This is a Rust HAL for the BL702 microcontroller.
//!
//! It implements the [`embedded-hal`] traits for peripherals, where such traits exist.
//!
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
//!
//! # Usage
//!
//!
//! ## Commonly used setup
//!
//! ```rust
//! // Get access to the device specific peripherals from the peripheral access crate
//! let dp = pac::Peripherals::take().unwrap();
//! let mut parts = dp.GLB.split();
//!
//! ```
//!
//!
//! To avoid the linker to complain about missing symbols please add `hal_defaults.x` to `.cargo/config` like this
//! ```toml
//! rustflags = [
//!   "-C", "link-arg=-Tmemory.x",
//!   "-C", "link-arg=-Tlink.x",
//!   "-C", "link-arg=-Thal_defaults.x",
//! ]
//! ```
//!

#![no_std]

pub use bl702_pac as pac;

pub mod delay;
pub mod gpio;

/// HAL crate prelude
pub mod prelude {
    pub use crate::gpio::GlbExt as _bl702_hal_gpio_GlbExt;
    pub use embedded_time::rate::Extensions;
}

/// System frequency (constant since we don't have clocks yet)
pub const SYSFREQ: u32 = 144_000_000;
