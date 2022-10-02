#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]

use embedded_time::rate::Hertz;
use crate::{system::{
    glb::{self, *},
    hbn::{
        self, HBN_32K_CLK_Type, HBN_32K_Sel, HBN_Power_On_Xtal_32K, HBN_Set_XCLK_CLK_Sel,
        HBN_XCLK_CLK_Type,
    },
    pds,
}, gpio::ClkCfg};

pub const BSP_FCLK_DIV: u8 = 0;
pub const BSP_BCLK_DIV: u8 = 1;

#[repr(C)]
pub enum system_clock_type {
    ///  clock source before fclk_div
    SYSTEM_CLOCK_ROOT_CLOCK = 0,
    ///  clock source after fclk_div
    SYSTEM_CLOCK_FCLK,
    ///  clock source after bclk_div
    SYSTEM_CLOCK_BCLK,
    ///  xtal clock
    SYSTEM_CLOCK_XCLK,
    ///  32K clock
    SYSTEM_CLOCK_32K_CLK,
    ///  audio PLL clock
    SYSTEM_CLOCK_AUPLL,
}

/// System bus frequency
pub const SYSFREQ: u32 = 144_000_000;
/// External high-speed crystal frequency
pub const XTAL_FREQ: u32 = 32_000_000;
/// UART peripheral clock frequency when PLL selected
pub const UART_PLL_FREQ: u32 = 96_000_000;

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum SysclkFreq {
    Pll144Mhz = 144_000_000,
}



/// Frozen clock frequencies
///
/// The existance of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    sysclk: Hertz,
    uart_clk: Hertz,
}

impl Clocks {
    pub fn new() -> Self {
        Clocks {
            sysclk: Hertz(SYSFREQ),
            uart_clk: Hertz(UART_PLL_FREQ),
        }
    }

    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    pub const fn uart_clk(&self) -> Hertz {
        self.uart_clk
    }
}

impl Default for Clocks {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ClockConfig {
    sysclk: SysclkFreq,
}

impl ClockConfig {
    /// Create initial clock config
    pub fn new() -> Self {
        ClockConfig {
            sysclk: SysclkFreq::Pll144Mhz,
        }
    }

    /// Calculate and balance clock registers to configure into the given clock value.
    /// Will choose closest valid value if it can't accurately select a frequency
    pub fn freeze(self, _clk_cfg: &mut ClkCfg) -> Clocks {
        let pll_enabled = true;
        let sysclk = self.sysclk;
        let uart_clk_div = 1; // leave uart clock at 96mhz

        unsafe { hbn::ptr() }
            .hbn_glb
            .modify(|_, w| w.hbn_uart_clk_sel().bit(pll_enabled));

        // Write UART clock divider
        unsafe { glb::ptr() }.clk_cfg2.modify(|_, w| unsafe {
            w.uart_clk_div()
                .bits(uart_clk_div - 1_u8)
                .uart_clk_en()
                .set_bit()
        });

        Clocks {
            sysclk: Hertz(sysclk as u32),
            uart_clk: Hertz(UART_PLL_FREQ),
        }
    }
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// This is late system init, called to reconfigure clocks as per users configuration
pub fn board_clock_init() {
    system_clock_init();
    peripheral_clock_init();
}

/// Set up core clocks
pub fn system_clock_init() {
    // select root clock
    GLB_Set_System_CLK(
        GLB_DLL_XTAL_Type::GLB_DLL_XTAL_32M,
        GLB_SYS_CLK_Type::GLB_SYS_CLK_DLL144M,
    );

    // set fclk/hclk and bclk clock
    GLB_Set_System_CLK_Div(BSP_FCLK_DIV, BSP_BCLK_DIV);
    // Set MTimer the same frequency as SystemCoreClock
    GLB_Set_MTimer_CLK(
        1,
        GLB_MTIMER_CLK_Type::GLB_MTIMER_CLK_BCLK,
        mtimer_get_clk_src_div() as u8,
    );

    // TODO: set audio PLL
    //  PDS_Set_Audio_PLL_Freq(BSP_AUDIO_PLL_CLOCK_SOURCE - ROOT_CLOCK_SOURCE_AUPLL_12288000_HZ);
    HBN_Power_On_Xtal_32K();
    HBN_32K_Sel(HBN_32K_CLK_Type::HBN_32K_XTAL);

    HBN_Set_XCLK_CLK_Sel(HBN_XCLK_CLK_Type::HBN_XCLK_CLK_XTAL);
}

/// Disable all peripheral clocks, then re-enable any that we're using  
/// TODO: move clock enable into peripheral drivers
pub fn peripheral_clock_init() {
    peripheral_clock_gate_all();
    unsafe {
        glb::ptr().cgen_cfg1.modify(|_, w| {
            w.uart0().set_bit();
            w.uart1().set_bit();
            w
        });
    }
}

/// This is early system init - called from preinit in the C SDK
pub fn system_init() {
    unsafe { riscv::interrupt::disable() };
    let pds = unsafe { pds::ptr() };
    let glb = unsafe { glb::ptr() };
    let hbn = unsafe { hbn::ptr() };
    let efuse0 = unsafe { &*bl702_pac::EF_DATA_0::ptr() };
    pds.pds_int.modify(|_r, w| {
        w.cr_pds_wake_int_mask().set_bit(); // mask pds wakeup
        w.cr_pds_rf_done_int_mask().set_bit(); // mask rf done
        w.cr_pds_pll_done_int_mask().set_bit(); // mask all pds wakeup source int
        unsafe {
            // mask all pds wakeup source int
            w.cr_pds_wakeup_src_en().bits(0);
        }
        w
    });

    // GLB_Set_EM_Sel(GLB_EM_0KB);
    glb.seam_misc.modify(|_r, w| unsafe { w.em_sel().bits(0) });

    /* Restore default setting*/
    /* GLB_UART_Sig_Swap_Set(UART_SIG_SWAP_NONE); */
    glb.glb_parm
        .modify(|_r, w| unsafe { w.uart_swap_set().bits(0) });

    /* fix 57.6M */
    if system_frequency() == 57 * 6000 * 1000 {
        unsafe {
            hbn.hbn_rsv2.write_with_zero(|w| {
                // Add 0.5 for const rounding
                w.hbn_rsv2().bits((57.6 * 1000.0 * 1000.0 + 0.5) as u32)
            });
        }
    }

    const CLIC_HART0_ADDR: usize = 0x02800000;
    const CLIC_INTIP: usize = 0x000;
    const CLIC_INTIE: usize = 0x400;
    const IRQ_NUM_BASE: usize = 16;
    const IRQ_QTY: usize = 64;
    const IRQ_ITER_END: usize = (IRQ_NUM_BASE + IRQ_QTY + 2) / 4;
    // TODO: create HAL CLIC interface rather than interact directly here
    // Clear all interrupts
    let clic_e = (CLIC_HART0_ADDR + CLIC_INTIE) as *mut usize;
    for i in 0..IRQ_ITER_END {
        unsafe { clic_e.wrapping_add(i).write_volatile(0) };
    }
    let clic_p = (CLIC_HART0_ADDR + CLIC_INTIP) as *mut usize;
    for i in 0..IRQ_ITER_END {
        unsafe { clic_p.wrapping_add(i).write_volatile(0) };
    }

    // TODO: update SVD with these fields
    // SF io select from efuse value
    let fuse = efuse0.ef_key_slot_5_w2.read().ef_key_slot_5_w2().bits();
    let flash_cfg = (fuse >> 26) & 7;
    let psram_cfg = (fuse >> 24) & 3;

    let is_internal_flash = flash_cfg == 1 || flash_cfg == 2;
    let is_internal_psram = psram_cfg == 1;

    glb.gpio_use_psram__io.modify(|_r, w| {
        unsafe {
            if is_internal_flash && !is_internal_psram {
                w.bits(0x3f);
            } else {
                w.bits(0);
            }
        }
        w
    });

    // TODO: register USB handler (if this isn't done at link time)
    // #ifdef BFLB_EFLASH_LOADER
    //     Interrupt_Handler_Register(USB_IRQn, USB_DoNothing_IRQHandler);
    // #endif

    // HBN_BOR_CFG_Type borCfg = { 0 /* pu_bor */, 0 /* irq_bor_en */, 1 /* bor_vth */, 0 /* bor_sel */ };
    hbn.hbn_irq_mode.modify(|_r, w| {
        w.irq_bor_en().clear_bit();
        w
    });

    hbn.hbn_misc.modify(|_r, w| {
        w.pu_bor().clear_bit();
        w.bor_vth().set_bit();
        w.bor_sel().clear_bit();
        w
    });

    unsafe { riscv::interrupt::enable() };
}

pub fn system_frequency() -> u32 {
    let hbn = unsafe { &*bl702_pac::HBN::ptr() };
    hbn.hbn_rsv2.read().hbn_rsv2().bits()
}

fn mtimer_get_clk_src_div() -> u32 {
    system_clock_get(system_clock_type::SYSTEM_CLOCK_BCLK) / 1000 / 1000 - 1
}

fn system_clock_get(t: system_clock_type) -> u32 {
    let clksel = GLB_Get_Root_CLK_Sel();
    match t {
        system_clock_type::SYSTEM_CLOCK_ROOT_CLOCK => {
            if clksel == GLB_ROOT_CLK_Type::GLB_ROOT_CLK_RC32M
                || clksel == GLB_ROOT_CLK_Type::GLB_ROOT_CLK_XTAL
            {
                32_000_000
            } else {
                let pll_sel = unsafe { glb::ptr().clk_cfg0.read().reg_pll_sel().bits() };
                match pll_sel {
                    0 => 57_600_000,
                    1 => 96_000_000,
                    2 => 144_000_000,
                    _ => 0,
                }
            }
        }
        system_clock_type::SYSTEM_CLOCK_FCLK => {
            system_clock_get(system_clock_type::SYSTEM_CLOCK_ROOT_CLOCK)
                / (GLB_Get_HCLK_Div() as u32 + 1)
        }
        system_clock_type::SYSTEM_CLOCK_BCLK => {
            system_clock_get(system_clock_type::SYSTEM_CLOCK_ROOT_CLOCK)
                / (GLB_Get_HCLK_Div() as u32 + 1)
                / (GLB_Get_BCLK_Div() as u32 + 1)
        }
        system_clock_type::SYSTEM_CLOCK_XCLK => 32_000_000,
        system_clock_type::SYSTEM_CLOCK_32K_CLK => 32_000,
        // TODO: lookup!
        system_clock_type::SYSTEM_CLOCK_AUPLL => 12_288_000,
    }
}
