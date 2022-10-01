MEMORY
{
    ROM       (rx)    : ORIGIN = 0x21000000, LENGTH = 128K
    CACHE     (wxa)   : ORIGIN = 0x42010000, LENGTH = 4K
    TCM_OCRAM (wxa)   : ORIGIN = 0x42011000, LENGTH = 124K
    FLASH     (rxa!w) : ORIGIN = 0x23000000, LENGTH = 2M
    HBNRAM    (wxa)   : ORIGIN = 0x40010000, LENGTH = 4K
}

REGION_ALIAS("REGION_TEXT", TCM_OCRAM);
REGION_ALIAS("REGION_RODATA", TCM_OCRAM);
REGION_ALIAS("REGION_DATA", TCM_OCRAM);
REGION_ALIAS("REGION_BSS", TCM_OCRAM);
REGION_ALIAS("REGION_HEAP", TCM_OCRAM);
REGION_ALIAS("REGION_STACK", TCM_OCRAM);
