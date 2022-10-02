#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]

use crate::system::romfunc::{data::ROM_API_INDEX_e, rom_fn_ptr};
use bl702_pac;

/// Direct register pointer.
///
/// # Safety
/// only use this to access registers you already have exclusive access to
pub unsafe fn ptr() -> &'static bl702_pac::pds::RegisterBlock {
    &*bl702_pac::PDS::ptr()
}

/// PLL XTAL type definition
#[repr(C)]
pub enum PDS_PLL_XTAL_Type {
    /// XTAL is none
    PDS_PLL_XTAL_NONE = 0,
    /// XTAL is 32M
    PDS_PLL_XTAL_32M = 1,
    /// XTAL is RC32M
    PDS_PLL_XTAL_RC32M = 2,
}

/// PLL output clock type definition
#[repr(C)]
pub enum PDS_PLL_CLK_Type {
    /// PLL output clock:480M
    PDS_PLL_CLK_480M = 0,
    /// PLL output clock:240M
    PDS_PLL_CLK_240M = 1,
    /// PLL output clock:192M
    PDS_PLL_CLK_192M = 2,
    /// PLL output clock:160M
    PDS_PLL_CLK_160M = 3,
    /// PLL output clock:120M
    PDS_PLL_CLK_120M = 4,
    /// PLL output clock:96M
    PDS_PLL_CLK_96M = 5,
    /// PLL output clock:80M
    PDS_PLL_CLK_80M = 6,
    /// PLL output clock:48M
    PDS_PLL_CLK_48M = 7,
    /// PLL output clock:32M
    PDS_PLL_CLK_32M = 8,
}

#[inline]
fn pds_power_on_pll_rom(xtal: PDS_PLL_XTAL_Type) {
    let romdriver_pds_power_on_pll = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(usize) -> usize>(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_PDS_Power_On_PLL,
        ))
    };

    // 0 == success, 1 == failure, 2 == timeout
    let pll_success = romdriver_pds_power_on_pll(xtal as usize);
    assert_eq!(pll_success, 0);
}
