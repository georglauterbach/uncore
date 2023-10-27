/* SPDX-License-Identifier: GPL-3.0-or-later */

/* This linker script was created by merging the information of multipple    */
/* different linker scripts:                                                 */
/* - `riscv64-unknown-elf-ld --verbose`
/* - https://mcyoung.xyz/2021/06/01/linker-script
/* - https://github.com/twilco/riscv-from-scratch
/* - https://github.com/sgmarz/osblog

/* We define that the architecture we are building for is RISC-V.            */
/* The value we use if valid for the 32bit and 64bit versions.               */
OUTPUT_ARCH(elf64-littleriscv)
OUTPUT_FORMAT(elf64-littleriscv)

/* We define the entrypoint, which is the _start function declared           */
/* in Assembly in init.S.                                                    */
ENTRY(__function__start)

/* The linker can search for archive libraries and ld control scripts here.  */
SEARCH_DIR(/usr/lib/riscv64-unknown-elf/lib);

/* To see the memory map's layout, visit                                     */
/* https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c                  */
/* -                                                                         */
/* There might be other boot ROMs at different addresses, but                */
/* their job is to get to this point.                                        */
MEMORY
{
  /* The RAM region is defined to be read-write ('rw'), executable           */
  /* ('x'), and allocatable ('a'). The memory starts at adress               */
  /* '0x8000_0000'. Technically, the size is arbitrary, and only             */
  /* bounded by the parameter that QEMU received for '-m'. By                */
  /* specifying the size, the linker can double-check that                   */
  /* everything fits.                                                        */
   ram (rwxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

REGION_ALIAS("REGION_TEXT", ram);
REGION_ALIAS("REGION_RODATA", ram);
REGION_ALIAS("REGION_DATA", ram);
REGION_ALIAS("REGION_BSS", ram);
REGION_ALIAS("REGION_HEAP", ram);
REGION_ALIAS("REGION_STACK", ram);

SECTIONS
{
  /* Read-only sections are merged into the text segment.                    */
  PROVIDE(__address__executable_start = SEGMENT_START("text-segment", ORIGIN(ram)));
  . = SEGMENT_START("text-segment", ORIGIN(ram)) + SIZEOF_HEADERS;

  .interp            : { *(.interp)                 }
  .hash              : { *(.hash)                   }
  .dynsym            : { *(.dynsym)                 }
  .dynstr            : { *(.dynstr)                 }
  .rela.dyn          : {
    *(.rela.init)
    *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)
    *(.rela.fini)
    *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)
    *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)
    *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)
    *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)
    *(.rela.ctors)
    *(.rela.dtors)
    *(.rela.got)
    *(.rela.sdata .rela.sdata.* .rela.gnu.linkonce.s.*)
    *(.rela.sbss .rela.sbss.* .rela.gnu.linkonce.sb.*)
    *(.rela.sdata2 .rela.sdata2.* .rela.gnu.linkonce.s2.*)
    *(.rela.sbss2 .rela.sbss2.* .rela.gnu.linkonce.sb2.*)
    *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)
    PROVIDE_HIDDEN(__rela_iplt_start = .);
    *(.rela.iplt)
    PROVIDE_HIDDEN(__rela_iplt_end = .);
  }
  .rela.plt          : { *(.rela.plt)               }
  .init              : { KEEP(*(SORT_NONE(.init)))  }
  .plt               : { *(.plt)                    }
  .iplt              : { *(.iplt)                   }
  .text              : {
    *(.text.unlikely .text.*_unlikely .text.unlikely.*)
    *(.text.exit .text.exit.*)
    *(.text.startup .text.startup.*)
    *(.text.hot .text.hot.*)
    *(.text .stub .text.* .gnu.linkonce.t.*)
  }
  .fini              : { KEEP(*(SORT_NONE(.fini))) }

  PROVIDE(__etext = .);
  PROVIDE(_etext = .);
  PROVIDE(etext = .);

  .rodata            : { *(.rodata .rodata.* .gnu.linkonce.r.*) }
  .rodata1           : { *(.rodata1) }
  .sdata2            : { *(.sdata2 .sdata2.* .gnu.linkonce.s2.*) }
  .sbss2             : { *(.sbss2 .sbss2.* .gnu.linkonce.sb2.*) }
  .eh_frame_hdr      : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }
  .eh_frame          : ONLY_IF_RO { KEEP(*(.eh_frame)) *(.eh_frame.*) }

  /* Adjust the address for the data segment. We want to adjust up to        */
  /* the same address within the page on the next page up.                   */
  . = DATA_SEGMENT_ALIGN(CONSTANT(MAXPAGESIZE), CONSTANT(COMMONPAGESIZE));

  /* Exception handling  */
  .eh_frame          : ONLY_IF_RW { KEEP(*(.eh_frame)) *(.eh_frame.*) }
  .gnu_extab         : ONLY_IF_RW { *(.gnu_extab) }
  .gcc_except_table  : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }
  .exception_ranges  : ONLY_IF_RW { *(.exception_ranges*) }
  /* Thread Local Storage sections  */
  .tdata             : {
    PROVIDE_HIDDEN (__tdata_start = .);
    *(.tdata .tdata.* .gnu.linkonce.td.*)
  }
  .tbss              : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }
  .preinit_array     : {
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP(*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);
  }
  .init_array        : {
    PROVIDE_HIDDEN (__init_array_start = .);
    KEEP(*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
    KEEP(*(.init_array EXCLUDE_FILE(*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o) .ctors))
    PROVIDE_HIDDEN (__init_array_end = .);
  }
  .fini_array        : {
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP(*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
    KEEP(*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
    PROVIDE_HIDDEN (__fini_array_end = .);
  }
  .ctors             : {
    /* gcc uses crtbegin.o to find the start of the constructors, so we make sure it is first.  Because this is a wildcard, it doesn't matter if the user does not actually link against crtbegin.o; the linker won't look for a file to match a wildcard.  The wildcard also means that it doesn't matter which directory crtbegin.o is in. */
    KEEP(*crtbegin.o(.ctors))
    KEEP(*crtbegin?.o(.ctors))
    /* We don't want to include the .ctor section from the crtend.o file until after the sorted ctors. The .ctor section from the crtend file contains the end of ctors marker and it must be last. */
    KEEP(*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
    KEEP(*(SORT(.ctors.*)))
    KEEP(*(.ctors))
  }
  .dtors             : {
    KEEP(*crtbegin.o(.dtors))
    KEEP(*crtbegin?.o(.dtors))
    KEEP(*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
    KEEP(*(SORT(.dtors.*)))
    KEEP(*(.dtors))
  }
  .jcr               : { KEEP(*(.jcr)) }
  .data.rel.ro       : {
    *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*)
  }
  .dynamic           : { *(.dynamic) }
  . = DATA_SEGMENT_RELRO_END (0, .);

  .data              : {
    __DATA_BEGIN__ = .;
    *(.data .data.* .gnu.linkonce.d.*)
    SORT(CONSTRUCTORS)
  }
  .data1             : { *(.data1) }
  .got               : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }
  /* We want the small data sections together, so single-instruction offsets can access them all, and initialized data all before uninitialized, so we can shorten the on-disk segment size. */
  .sdata             : {
    __SDATA_BEGIN__ = .;
    *(.srodata.cst16) *(.srodata.cst8) *(.srodata.cst4) *(.srodata.cst2) *(.srodata .srodata.*)
    *(.sdata .sdata.* .gnu.linkonce.s.*)
  }
  _edata = .;
  PROVIDE(edata = .);
  . = .;

  __bss_start = .;
  .sbss              : {
    *(.dynsbss)
    *(.sbss .sbss.* .gnu.linkonce.sb.*)
    *(.scommon)
  }
  .bss               : {
   *(.dynbss)
   *(.bss .bss.* .gnu.linkonce.b.*)
   *(COMMON)
   /* Align here to ensure that the .bss section occupies space up to _end.  Align after .bss to ensure correct alignment even if the .bss section disappears because there are no input sections. FIXME: Why do we need it? When there is no .bss section, we do not pad the .data section. */
   . = ALIGN(. != 0 ? 64 / 8 : 1);
  }
  . = ALIGN(64 / 8);
  . = SEGMENT_START("ldata-segment", .);
  . = ALIGN(64 / 8);
  __BSS_END__ = .;
  __global_pointer$ = MIN(__SDATA_BEGIN__ + 0x800, MAX(__DATA_BEGIN__ + 0x800, __BSS_END__ - 0x800));
  _end = .;
  PROVIDE(end = .);
  . = DATA_SEGMENT_END (.);

  /* The kernel now has a stack size of 1M bytes, which is plenty            */
  PROVIDE(__stack_top = ORIGIN(ram) + LENGTH(ram) - 2M);

  /* DWARF debug sections. Symbols in the DWARF debugging sections           */
  /* are relative to the beginning of the section so we begin them at 0.     */
  /* DWARF 1                                                                 */
  .debug              0 : { *(.debug) }
  .line               0 : { *(.line) }
  /* GNU DWARF 1 Extensions                                                  */
  .debug_srcinfo      0 : { *(.debug_srcinfo) }
  .debug_sfnames      0 : { *(.debug_sfnames) }
  /* DWARF 1.1 and DWARF 2                                                   */
  .debug_aranges      0 : { *(.debug_aranges) }
  .debug_pubnames     0 : { *(.debug_pubnames) }
  /* DWARF 2                                                                 */
  .debug_info         0 : { *(.debug_info .gnu.linkonce.wi.*) }
  .debug_abbrev       0 : { *(.debug_abbrev) }
  .debug_line         0 : { *(.debug_line .debug_line.* .debug_line_end) }
  .debug_frame        0 : { *(.debug_frame) }
  .debug_str          0 : { *(.debug_str) }
  .debug_loc          0 : { *(.debug_loc) }
  .debug_macinfo      0 : { *(.debug_macinfo) }
  /* DWARF 3                                                                 */
  .debug_pubtypes     0 : { *(.debug_pubtypes) }
  .debug_ranges       0 : { *(.debug_ranges) }
  /* DWARF 5                                                                 */
  .debug_addr         0 : { *(.debug_addr) }
  .debug_line_str     0 : { *(.debug_line_str) }
  .debug_loclists     0 : { *(.debug_loclists) }
  .debug_macro        0 : { *(.debug_macro) }
  .debug_names        0 : { *(.debug_names) }
  .debug_rnglists     0 : { *(.debug_rnglists) }
  .debug_str_offsets  0 : { *(.debug_str_offsets) }
  .debug_sup          0 : { *(.debug_sup) }
  .ARM.attributes     0 : { KEEP (*(.ARM.attributes)) KEEP (*(.gnu.attributes)) }
  .note.gnu.arm.ident 0 : { KEEP (*(.note.gnu.arm.ident)) }

  /DISCARD/             : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}
