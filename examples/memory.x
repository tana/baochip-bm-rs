MEMORY
{
	/* SRAM. It needs to be named RAM for flip-link. */
	RAM : ORIGIN = 0x61000000, LENGTH = 2M
	/* RRAM is a nonvolatile memory for code and static data. */
	/* First 0x60000 bytes are reserved for bootloaders and 768 bytes for a signature. */
	/* Also last 152K bytes are used for security. */
	RRAM : ORIGIN = 0x60060000 + 768, LENGTH = 4M - 768 - 152K
	/* IFRAM is the only region that can be used for DMA. */
	IFRAM : ORIGIN = 0x50000000, LENGTH = 256K
}

REGION_ALIAS("REGION_TEXT", RRAM);
REGION_ALIAS("REGION_RODATA", RRAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);  /* This is necessary even if heap is not used */
REGION_ALIAS("REGION_STACK", RAM);
