use super::BL_Err_Type;

/// brief HBN UART clock type definition
#[repr(C)]
pub enum HBN_UART_CLK_Type {
    /// Select FCLK as UART clock
    HBN_UART_CLK_FCLK = 0,
    /// Select 96M as UART clock
    HBN_UART_CLK_96M,
}

/**
 *  @brief HBN 32K clock type definition
 */
pub enum HBN_32K_CLK_Type {
    /// HBN use rc 32k
    HBN_32K_RC = 0,
    /// HBN use xtal 32k
    HBN_32K_XTAL,
    /// HBN use dig 32k
    HBN_32K_DIG = 3,
}

/**
 *  @brief HBN xclk clock type definition
 */
pub enum HBN_XCLK_CLK_Type {
    /// use RC32M as xclk clock
    HBN_XCLK_CLK_RC32M,
    /// use XTAL as xclk clock
    HBN_XCLK_CLK_XTAL,
}

// romfunc
pub fn HBN_32K_Sel(clkType: HBN_32K_CLK_Type) -> BL_Err_Type {
    BL_Err_Type::SUCCESS
}

pub fn HBN_Set_XCLK_CLK_Sel(xClk: HBN_XCLK_CLK_Type) -> BL_Err_Type {
    BL_Err_Type::SUCCESS
}

// romfunc
pub fn HBN_Power_On_Xtal_32K() -> BL_Err_Type {
    BL_Err_Type::SUCCESS
}
