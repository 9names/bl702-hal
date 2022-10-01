use crate::system::{
    glb::{self, *},
    hbn::{
        self, HBN_32K_CLK_Type, HBN_32K_Sel, HBN_Power_On_Xtal_32K, HBN_Set_XCLK_CLK_Sel,
        HBN_XCLK_CLK_Type,
    },
    pds,
};

// const ROOT_CLOCK_SOURCE_XCLK:usize = 0;
// const ROOT_CLOCK_SOURCE_XCLK_ALT:usize = 1;
// const ROOT_CLOCK_SOURCE_PLL_57P6M:usize = 2;
// const ROOT_CLOCK_SOURCE_PLL_96M:usize   = 3;
// const ROOT_CLOCK_SOURCE_PLL_144M:usize  =4;

// #define XTAL_TYPE                  EXTERNAL_XTAL_32M
// #define XTAL_32K_TYPE              INTERNAL_RC_32K
// #define BSP_ROOT_CLOCK_SOURCE      ROOT_CLOCK_SOURCE_PLL_144M
// #define BSP_AUDIO_PLL_CLOCK_SOURCE ROOT_CLOCK_SOURCE_AUPLL_24000000_HZ

// #define BSP_FCLK_DIV 0
// #define BSP_BCLK_DIV 1
pub const BSP_FCLK_DIV: u8 = 0;
pub const BSP_BCLK_DIV: u8 = 1;

// this is called by set_system_clk, let's use that
// BL_Err_Type PDS_Enable_PLL_All_Clks(void);
// BL_Err_Type PDS_Enable_PLL_Clk(PDS_PLL_CLK_Type pllClk);

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
    SYSTEM_CLOCK_32K_CLK,
    SYSTEM_CLOCK_AUPLL,
}

fn system_clock_get(t: system_clock_type) -> u32 {
    let clksel = GLB_Get_Root_CLK_Sel();
    match t {
        system_clock_type::SYSTEM_CLOCK_ROOT_CLOCK => {
            32 * 1000 * 1000
            // if clksel == GLB_ROOT_CLK_Type::GLB_ROOT_CLK_RC32M  || clksel == GLB_ROOT_CLK_Type::GLB_ROOT_CLK_XTAL {
            //     32 * 1000 * 1000
            // }else {
            //     //TODO: some work
            //     32 * 1000 * 1000
            //     // let tmpVal = BL_RD_REG(GLB_BASE, GLB_CLK_CFG0);
            //     // tmpVal = BL_GET_REG_BITS_VAL(tmpVal, GLB_REG_PLL_SEL);
            //     // if (tmpVal == 0) {
            //     //     return 57.6 * 1000 * 1000;
            //     // } else if (tmpVal == 1) {
            //     //     return 96 * 1000 * 1000;
            //     // } else if (tmpVal == 2) {
            //     //     return 144 * 1000 * 1000;
            //     // } else {
            //     //     return 0;
            //     // }
            // }
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
        system_clock_type::SYSTEM_CLOCK_XCLK => 32000000,
        system_clock_type::SYSTEM_CLOCK_32K_CLK => 32000,
        // TODO: lookup!
        system_clock_type::SYSTEM_CLOCK_AUPLL => 12288000,
    }
}

fn mtimer_get_clk_src_div() -> u32 {
    return system_clock_get(system_clock_type::SYSTEM_CLOCK_BCLK) / 1000 / 1000 - 1;
}

pub fn system_clock_init() {
    // /*select root clock*/
    GLB_Set_System_CLK(
        GLB_DLL_XTAL_Type::GLB_DLL_XTAL_32M,
        GLB_SYS_CLK_Type::GLB_SYS_CLK_DLL144M,
    );

    // /*set fclk/hclk and bclk clock*/
    GLB_Set_System_CLK_Div(BSP_FCLK_DIV, BSP_BCLK_DIV);
    // /* Set MTimer the same frequency as SystemCoreClock */
    GLB_Set_MTimer_CLK(
        1,
        GLB_MTIMER_CLK_Type::GLB_MTIMER_CLK_BCLK,
        mtimer_get_clk_src_div() as u8,
    );

    //  PDS_Set_Audio_PLL_Freq(BSP_AUDIO_PLL_CLOCK_SOURCE - ROOT_CLOCK_SOURCE_AUPLL_12288000_HZ);
    HBN_Power_On_Xtal_32K();
    HBN_32K_Sel(HBN_32K_CLK_Type::HBN_32K_XTAL);

    HBN_Set_XCLK_CLK_Sel(HBN_XCLK_CLK_Type::HBN_XCLK_CLK_XTAL);
}

pub fn peripheral_clock_init() {
    peripheral_clock_gate_all();
    unsafe {
        glb::ptr().cgen_cfg1.modify(|r, w| {
            w.uart0().set_bit();
            w.uart1().set_bit();
            w
        });
    }
}

pub fn board_clock_init() {
    system_clock_init();
    peripheral_clock_init();
}

pub fn SystemInit() {
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
            w.cr_pds_wakeup_src_en().bits(0);
        }
        w
        // w.cr_pds_int_clr()
    });

    /* GLB_Set_EM_Sel(GLB_EM_0KB); */
    glb.seam_misc.modify(|_r, w| {
        unsafe { w.em_sel().bits(0) };
        w
    });

    /* Restore default setting*/
    /* GLB_UART_Sig_Swap_Set(UART_SIG_SWAP_NONE); */
    glb.glb_parm.modify(|_r, w| {
        unsafe {
            w.uart_swap_set().bits(0);
        }
        w
    });

    /* fix 57.6M */
    if system_frequency() == 57 * 6000 * 1000 {
        unsafe {
            hbn.hbn_rsv2.write_with_zero(|w| {
                // Add 0.5 for const rounding
                w.hbn_rsv2().bits((57.6 * 1000.0 * 1000.0 + 0.5) as u32);
                w
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
    let clic_e = (CLIC_HART0_ADDR + CLIC_INTIE) as *mut usize;
    for i in 0..IRQ_ITER_END {
        unsafe { clic_e.wrapping_add(i).write_volatile(0) };
    }
    let clic_p = (CLIC_HART0_ADDR + CLIC_INTIP) as *mut usize;
    for i in 0..IRQ_ITER_END {
        unsafe { clic_p.wrapping_add(i).write_volatile(0) };
    }

    // TODO: update SVD with these fields
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

// void bl_show_flashinfo(void)
// {
//     SPI_Flash_Cfg_Type flashCfg;
//     uint8_t *pFlashCfg = NULL;
//     uint32_t flashCfgLen = 0;
//     uint32_t flashJedecId = 0;

//     flashJedecId = flash_get_jedecid();
//     flash_get_cfg(&pFlashCfg, &flashCfgLen);
//     arch_memcpy((void *)&flashCfg, pFlashCfg, flashCfgLen);
//     MSG("show flash cfg:\r\n");
//     MSG("jedec id   0x%06X\r\n", flashJedecId);
//     MSG("mid            0x%02X\r\n", flashCfg.mid);
//     MSG("iomode         0x%02X\r\n", flashCfg.ioMode);
//     MSG("clk delay      0x%02X\r\n", flashCfg.clkDelay);
//     MSG("clk invert     0x%02X\r\n", flashCfg.clkInvert);
//     MSG("read reg cmd0  0x%02X\r\n", flashCfg.readRegCmd[0]);
//     MSG("read reg cmd1  0x%02X\r\n", flashCfg.readRegCmd[1]);
//     MSG("write reg cmd0 0x%02X\r\n", flashCfg.writeRegCmd[0]);
//     MSG("write reg cmd1 0x%02X\r\n", flashCfg.writeRegCmd[1]);
//     MSG("qe write len   0x%02X\r\n", flashCfg.qeWriteRegLen);
//     MSG("cread support  0x%02X\r\n", flashCfg.cReadSupport);
//     MSG("cread code     0x%02X\r\n", flashCfg.cReadMode);
//     MSG("burst wrap cmd 0x%02X\r\n", flashCfg.burstWrapCmd);
//     MSG("-------------------\r\n");
// }
