use crate::system::romfunc::{data::ROM_API_INDEX_e, rom_fn_ptr};

use super::{hbn::HBN_UART_CLK_Type, BL_Err_Type};
use crate::pac;

/// Direct register pointer.
///
/// # Safety
/// only use this to access registers you already have exclusive access to
pub unsafe fn ptr() -> &'static bl702_pac::glb::RegisterBlock {
    &*pac::GLB::ptr()
}

///brief PLL XTAL type definition
#[repr(C)]
pub enum GLB_PLL_XTAL_Type {
    /// XTAL is none
    GLB_PLL_XTAL_NONE = 0,
    /// XTAL is 24M
    GLB_PLL_XTAL_24M = 1,
    /// XTAL is 32M
    GLB_PLL_XTAL_32M = 2,
    /// XTAL is 38.4M
    GLB_PLL_XTAL_38P4M = 3,
    /// XTAL is 40M
    GLB_PLL_XTAL_40 = 4,
    /// XTAL is 26M
    GLB_PLL_XTAL_26M = 5,
    /// XTAL is RC32M
    GLB_PLL_XTAL_RC326 = 6,
}

/// PLL XTAL type definition
#[repr(C)]
pub enum GLB_DLL_XTAL_Type {
    /// XTAL is none
    GLB_DLL_XTAL_NONE,
    /// XTAL is 32M
    GLB_DLL_XTAL_32M,
    /// XTAL is RC32M
    GLB_DLL_XTAL_RC32M,
}

/// GLB system clock type definition
#[repr(C)]
pub enum GLB_SYS_CLK_Type {
    /// use RC32M as system clock frequency
    GLB_SYS_CLK_RC32M,
    /// use XTAL as system clock
    GLB_SYS_CLK_XTAL,
    /// use DLL output 57.6M as system clock
    GLB_SYS_CLK_DLL57P6M,
    /// use DLL output 96M as system clock
    GLB_SYS_CLK_DLL96M,
    /// use DLL output 144M as system clock, PLL120M not recommend
    GLB_SYS_CLK_DLL144M,
}

/// GLB RTC clock type definition
#[repr(C)]
pub enum GLB_MTIMER_CLK_Type {
    /// BUS clock
    GLB_MTIMER_CLK_BCLK,
    /// 32KHz
    GLB_MTIMER_CLK_32K,
}

///brief GLB root clock type definition
#[repr(C)]
#[derive(PartialEq)]
pub enum GLB_ROOT_CLK_Type {
    ///root clock select RC32M
    GLB_ROOT_CLK_RC32M,
    ///root clock select XTAL
    GLB_ROOT_CLK_XTAL,
    ///root clock select DLL others, PLL120M not recommend
    GLB_ROOT_CLK_DLL,
}

// romfunc ((GLB_ROOT_CLK_Type(*)(void))ROM_APITABLE[ROM_API_INDEX_GLB_Get_Root_CLK_Sel])
pub fn GLB_Get_Root_CLK_Sel() -> GLB_ROOT_CLK_Type {
    unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> GLB_ROOT_CLK_Type>(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_GLB_Get_Root_CLK_Sel,
        ))()
    }
}

// romfunc ((uint8_t(*)(void))ROM_APITABLE[ROM_API_INDEX_GLB_Get_HCLK_Div])
pub fn GLB_Get_HCLK_Div() -> u8 {
    unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> u8>(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_GLB_Get_HCLK_Div,
        ))()
    }
}

// romfunc ((uint8_t(*)(void))ROM_APITABLE[ROM_API_INDEX_GLB_Get_BCLK_Div])
pub fn GLB_Get_BCLK_Div() -> u8 {
    unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> u8>(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_GLB_Get_HCLK_Div,
        ))()
    }
}

pub fn GLB_Set_UART_CLK(enable: u8, clkSel: HBN_UART_CLK_Type, div: u8) -> BL_Err_Type {
    BL_Err_Type::SUCCESS
}

// romfunc ((BL_Err_Type(*)(GLB_DLL_XTAL_Type xtalType, GLB_SYS_CLK_Type clkFreq))ROM_APITABLE[ROM_API_INDEX_GLB_Set_System_CLK])
pub fn GLB_Set_System_CLK(xtalType: GLB_DLL_XTAL_Type, clkFreq: GLB_SYS_CLK_Type) -> BL_Err_Type {
    unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(GLB_DLL_XTAL_Type, GLB_SYS_CLK_Type) -> BL_Err_Type,
        >(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_GLB_Set_System_CLK,
        ))(xtalType, clkFreq)
    }
}

// romfunc ((BL_Err_Type(*)(uint8_t hclkDiv, uint8_t bclkDiv))ROM_APITABLE[ROM_API_INDEX_GLB_Set_System_CLK_Div])
pub fn GLB_Set_System_CLK_Div(clkDiv: u8, bclkDiv: u8) -> BL_Err_Type {
    unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8, u8) -> BL_Err_Type>(rom_fn_ptr(
            ROM_API_INDEX_e::ROM_API_INDEX_GLB_Set_System_CLK_Div,
        ))(clkDiv, bclkDiv)
    }
}

pub fn GLB_Set_MTimer_CLK(enable: u8, clkSel: GLB_MTIMER_CLK_Type, div: u8) -> BL_Err_Type {
    BL_Err_Type::SUCCESS
}
