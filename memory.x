/* memory.x — translated from your imxrt1062_t41.ld MEMORY block */

MEMORY
{
  ITCM  (rwx) : ORIGIN = 0x00000000, LENGTH = 512K
  DTCM  (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
  RAM   (rwx) : ORIGIN = 0x20200000, LENGTH = 512K   /* OCRAM */
  FLASH (rwx) : ORIGIN = 0x60000000, LENGTH = 7936K  /* QSPI */
  ERAM  (rwx) : ORIGIN = 0x70000000, LENGTH = 16384K /* External RAM */
}

/* ---- Region aliases used by imxrt-rt’s link.x (execute from ITCM, load from FLASH) ---- */
REGION_ALIAS("REGION_TEXT",       ITCM);
REGION_ALIAS("REGION_TEXT_LOAD",  FLASH);
REGION_ALIAS("REGION_RODATA",     ITCM);   /* rodata landed with text in ITCM after copy */
REGION_ALIAS("REGION_DATA",       DTCM);
REGION_ALIAS("REGION_BSS",        DTCM);
REGION_ALIAS("REGION_HEAP",       RAM);
REGION_ALIAS("REGION_STACK",      DTCM);

/* Optional helpers for your own code (not used by the runtime by default) */
PROVIDE(_eram_start = ORIGIN(ERAM));
PROVIDE(_eram_size  = LENGTH(ERAM));
PROVIDE(_flash_start = ORIGIN(FLASH));
PROVIDE(_flash_size  = LENGTH(FLASH));
