MEMORY
{
    ROM       (rx)    : ORIGIN = 0x21000000, LENGTH = 128K
    TCM_OCRAM (wxa)   : ORIGIN = 0x42014000, LENGTH = 112K
    FLASH     (rxa!w) : ORIGIN = 0x23000000, LENGTH = 2M
    HBNRAM    (wxa)   : ORIGIN = 0x40010000, LENGTH = 4K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", TCM_OCRAM);
REGION_ALIAS("REGION_BSS", TCM_OCRAM);
REGION_ALIAS("REGION_HEAP", TCM_OCRAM);
REGION_ALIAS("REGION_STACK", TCM_OCRAM);

/*
Notes:
  - Actual RAM mapping seems to be something like the table below (TODO: verify)
*/

/*
  flash (rxai!w) : ORIGIN = 0x23000000, LENGTH = (2M)
  CACHE     (wxa): ORIGIN = 0x42010000, LENGTH = 4K // Note: this seems unlikely - maybe 0x4000?
  tcm      (wxa) : ORIGIN = 0x42014000, LENGTH = (48K)
  ocram_1  (wxa) : ORIGIN = 0x42020000, LENGTH = (24K)  // will not be initialized in bootrom
  ocram_2  (wxa) : ORIGIN = 0x42026000, LENGTH = (16K)  // will be initialized in bootrom
  ocram_3  (wxa) : ORIGIN = 0x4202A000, LENGTH = (24K - __EM_SIZE)  // will not be initialized in bootrom
  hbnram   (wxa) : ORIGIN = 0x40010000, LENGTH = (4K)
*/
