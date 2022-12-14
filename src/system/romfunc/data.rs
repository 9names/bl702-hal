#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

pub const ROM_APITABLE_ADDR: usize = 0x2101_8800;
pub const ROMAPI_INDEX_SECT_SIZE: usize = 0x800;
pub const ROMAPI_INDEX_MAX: usize = ROMAPI_INDEX_SECT_SIZE / 4 - 1;
pub const ROM_API_INDEX_FUNC_START: usize = 4;

#[repr(usize)]
pub enum ROM_API_INDEX_e {
    ROM_API_INDEX_REV = 0,
    ROM_API_INDEX_AON_Power_On_MBG = ROM_API_INDEX_FUNC_START,
    ROM_API_INDEX_AON_Power_Off_MBG,
    ROM_API_INDEX_AON_Power_On_XTAL,
    ROM_API_INDEX_AON_Set_Xtal_CapCode,
    ROM_API_INDEX_AON_Power_Off_XTAL,
    ROM_API_INDEX_ASM_Delay_Us,
    ROM_API_INDEX_BL702_Delay_US,
    ROM_API_INDEX_BL702_Delay_MS,
    ROM_API_INDEX_BL702_MemCpy,
    ROM_API_INDEX_BL702_MemCpy4,
    ROM_API_INDEX_BL702_MemCpy_Fast,
    ROM_API_INDEX_ARCH_MemCpy_Fast,
    ROM_API_INDEX_BL702_MemSet,
    ROM_API_INDEX_BL702_MemSet4,
    ROM_API_INDEX_BL702_MemCmp,
    ROM_API_INDEX_BFLB_Soft_CRC32,
    ROM_API_INDEX_GLB_Get_Root_CLK_Sel,
    ROM_API_INDEX_GLB_Set_System_CLK_Div,
    ROM_API_INDEX_GLB_Get_BCLK_Div,
    ROM_API_INDEX_GLB_Get_HCLK_Div,
    ROM_API_INDEX_Update_SystemCoreClockWith_XTAL,
    ROM_API_INDEX_GLB_Set_System_CLK,
    ROM_API_INDEX_System_Core_Clock_Update_From_RC32M,
    ROM_API_INDEX_GLB_Set_SF_CLK,
    ROM_API_INDEX_GLB_Power_Off_DLL,
    ROM_API_INDEX_GLB_Power_On_DLL,
    ROM_API_INDEX_GLB_Enable_DLL_All_Clks,
    ROM_API_INDEX_GLB_Enable_DLL_Clk,
    ROM_API_INDEX_GLB_Disable_DLL_All_Clks,
    ROM_API_INDEX_GLB_Disable_DLL_Clk,
    ROM_API_INDEX_GLB_SW_System_Reset,
    ROM_API_INDEX_GLB_SW_CPU_Reset,
    ROM_API_INDEX_GLB_SW_POR_Reset,
    ROM_API_INDEX_GLB_Select_Internal_Flash,
    ROM_API_INDEX_GLB_Swap_Flash_Pin,
    ROM_API_INDEX_GLB_Swap_Flash_CS_IO2_Pin,
    ROM_API_INDEX_GLB_Swap_Flash_IO0_IO3_Pin,
    ROM_API_INDEX_GLB_Select_Internal_PSram,
    ROM_API_INDEX_GLB_GPIO_Init,
    ROM_API_INDEX_GLB_GPIO_OUTPUT_Enable,
    ROM_API_INDEX_GLB_GPIO_OUTPUT_Disable,
    ROM_API_INDEX_GLB_GPIO_Set_HZ,
    ROM_API_INDEX_GLB_Deswap_Flash_Pin,
    ROM_API_INDEX_GLB_Select_External_Flash,
    ROM_API_INDEX_GLB_GPIO_Get_Fun,
    ROM_API_INDEX_EF_Ctrl_Busy,
    ROM_API_INDEX_EF_Ctrl_Sw_AHB_Clk_0,
    ROM_API_INDEX_EF_Ctrl_Load_Efuse_R0,
    ROM_API_INDEX_EF_Ctrl_Clear,
    ROM_API_INDEX_EF_Ctrl_Get_Trim_Parity,
    ROM_API_INDEX_EF_Ctrl_Read_RC32K_Trim,
    ROM_API_INDEX_EF_Ctrl_Read_RC32M_Trim,
    ROM_API_INDEX_PDS_Trim_RC32M,
    ROM_API_INDEX_PDS_Select_RC32M_As_PLL_Ref,
    ROM_API_INDEX_PDS_Select_XTAL_As_PLL_Ref,
    ROM_API_INDEX_PDS_Power_On_PLL,
    ROM_API_INDEX_PDS_Enable_PLL_All_Clks,
    ROM_API_INDEX_PDS_Disable_PLL_All_Clks,
    ROM_API_INDEX_PDS_Enable_PLL_Clk,
    ROM_API_INDEX_PDS_Disable_PLL_Clk,
    ROM_API_INDEX_PDS_Power_Off_PLL,
    ROM_API_INDEX_PDS_Reset,
    ROM_API_INDEX_PDS_Enable,
    ROM_API_INDEX_PDS_Auto_Time_Config,
    ROM_API_INDEX_PDS_Auto_Enable,
    ROM_API_INDEX_PDS_Manual_Force_Turn_Off,
    ROM_API_INDEX_PDS_Manual_Force_Turn_On,
    ROM_API_INDEX_HBN_Enable,
    ROM_API_INDEX_HBN_Reset,
    ROM_API_INDEX_HBN_GPIO_Dbg_Pull_Cfg,
    ROM_API_INDEX_HBN_Trim_RC32K,
    ROM_API_INDEX_HBN_Set_ROOT_CLK_Sel,
    ROM_API_INDEX_XIP_SFlash_State_Save,
    ROM_API_INDEX_XIP_SFlash_State_Restore,
    ROM_API_INDEX_XIP_SFlash_Erase_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_Write_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_Read_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_GetJedecId_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_GetDeviceId_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_GetUniqueId_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_Read_Via_Cache_Need_Lock,
    ROM_API_INDEX_XIP_SFlash_Read_With_Lock,
    ROM_API_INDEX_XIP_SFlash_Write_With_Lock,
    ROM_API_INDEX_XIP_SFlash_Erase_With_Lock,

    ROM_API_INDEX_SFlash_Init,
    ROM_API_INDEX_SFlash_SetSPIMode,
    ROM_API_INDEX_SFlash_Read_Reg,
    ROM_API_INDEX_SFlash_Write_Reg,
    ROM_API_INDEX_SFlash_Read_Reg_With_Cmd,
    ROM_API_INDEX_SFlash_Write_Reg_With_Cmd,
    ROM_API_INDEX_SFlash_Busy,
    ROM_API_INDEX_SFlash_Write_Enable,
    ROM_API_INDEX_SFlash_Qspi_Enable,
    ROM_API_INDEX_SFlash_Volatile_Reg_Write_Enable,
    ROM_API_INDEX_SFlash_Chip_Erase,
    ROM_API_INDEX_SFlash_Sector_Erase,
    ROM_API_INDEX_SFlash_Blk32_Erase,
    ROM_API_INDEX_SFlash_Blk64_Erase,
    ROM_API_INDEX_SFlash_Erase,
    ROM_API_INDEX_SFlash_Program,
    ROM_API_INDEX_SFlash_GetUniqueId,
    ROM_API_INDEX_SFlash_GetJedecId,
    ROM_API_INDEX_SFlash_GetDeviceId,
    ROM_API_INDEX_SFlash_Powerdown,
    ROM_API_INDEX_SFlash_Releae_Powerdown,
    ROM_API_INDEX_SFlash_Restore_From_Powerdown,
    ROM_API_INDEX_SFlash_SetBurstWrap,
    ROM_API_INDEX_SFlash_DisableBurstWrap,
    ROM_API_INDEX_SFlash_Software_Reset,
    ROM_API_INDEX_SFlash_Reset_Continue_Read,
    ROM_API_INDEX_SFlash_Set_IDbus_Cfg,
    ROM_API_INDEX_SFlash_IDbus_Read_Enable,
    ROM_API_INDEX_SFlash_Cache_Read_Enable,
    ROM_API_INDEX_SFlash_Cache_Read_Disable,
    ROM_API_INDEX_SFlash_Read,

    ROM_API_INDEX_L1C_Cache_Enable_Set,
    ROM_API_INDEX_L1C_Cache_Write_Set,
    ROM_API_INDEX_L1C_Cache_Flush,
    ROM_API_INDEX_L1C_Cache_Hit_Count_Get,
    ROM_API_INDEX_L1C_Cache_Miss_Count_Get,
    ROM_API_INDEX_L1C_Cache_Read_Disable,
    ROM_API_INDEX_L1C_Set_Wrap,
    ROM_API_INDEX_L1C_Set_Way_Disable,
    ROM_API_INDEX_L1C_IROM_2T_Access_Set,

    ROM_API_INDEX_SF_Ctrl_Enable,
    ROM_API_INDEX_SF_Ctrl_Psram_Init,
    ROM_API_INDEX_SF_Ctrl_Get_Clock_Delay,
    ROM_API_INDEX_SF_Ctrl_Set_Clock_Delay,
    ROM_API_INDEX_SF_Ctrl_Cmds_Set,
    ROM_API_INDEX_SF_Ctrl_Set_Owner,
    ROM_API_INDEX_SF_Ctrl_Disable,
    ROM_API_INDEX_SF_Ctrl_Select_Pad,
    ROM_API_INDEX_SF_Ctrl_Select_Bank,
    ROM_API_INDEX_SF_Ctrl_AES_Enable_BE,
    ROM_API_INDEX_SF_Ctrl_AES_Enable_LE,
    ROM_API_INDEX_SF_Ctrl_AES_Set_Region,
    ROM_API_INDEX_SF_Ctrl_AES_Set_Key,
    ROM_API_INDEX_SF_Ctrl_AES_Set_Key_BE,
    ROM_API_INDEX_SF_Ctrl_AES_Set_IV,
    ROM_API_INDEX_SF_Ctrl_AES_Set_IV_BE,
    ROM_API_INDEX_SF_Ctrl_AES_Enable,
    ROM_API_INDEX_SF_Ctrl_AES_Disable,
    ROM_API_INDEX_SF_Ctrl_Is_AES_Enable,
    ROM_API_INDEX_SF_Ctrl_Set_Flash_Image_Offset,
    ROM_API_INDEX_SF_Ctrl_Get_Flash_Image_Offset,
    ROM_API_INDEX_SF_Ctrl_Select_Clock,
    ROM_API_INDEX_SF_Ctrl_SendCmd,
    ROM_API_INDEX_SF_Ctrl_Flash_Read_Icache_Set,
    ROM_API_INDEX_SF_Ctrl_Psram_Write_Icache_Set,
    ROM_API_INDEX_SF_Ctrl_Psram_Read_Icache_Set,
    ROM_API_INDEX_SF_Ctrl_GetBusyState,
    ROM_API_INDEX_SF_Cfg_Deinit_Ext_Flash_Gpio,
    ROM_API_INDEX_SF_Cfg_Init_Ext_Flash_Gpio,
    ROM_API_INDEX_SF_Cfg_Get_Flash_Cfg_Need_Lock,
    ROM_API_INDEX_SF_Cfg_Init_Flash_Gpio,
    ROM_API_INDEX_SF_Cfg_Flash_Identify,

    ROM_API_INDEX_Psram_Init,
    ROM_API_INDEX_Psram_ReadReg,
    ROM_API_INDEX_Psram_WriteReg,
    ROM_API_INDEX_Psram_SetDriveStrength,
    ROM_API_INDEX_Psram_SetBurstWrap,
    ROM_API_INDEX_Psram_ReadId,
    ROM_API_INDEX_Psram_EnterQuadMode,
    ROM_API_INDEX_Psram_ExitQuadMode,
    ROM_API_INDEX_Psram_ToggleBurstLength,
    ROM_API_INDEX_Psram_SoftwareReset,
    ROM_API_INDEX_Psram_Set_IDbus_Cfg,
    ROM_API_INDEX_Psram_Cache_Write_Set,
    ROM_API_INDEX_Psram_Write,
    ROM_API_INDEX_Psram_Read,

    ROM_API_INDEX_FUNC_INVALID_START,

    ROM_API_INDEX_FUNC_LAST_ENTRY = ROMAPI_INDEX_MAX,
}
pub const ROM_API_INDEX_FUNC_LAST: usize = ROM_API_INDEX_e::ROM_API_INDEX_Psram_Read as usize;
