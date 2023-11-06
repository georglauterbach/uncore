/* SPDX-License-Identifier: GPL-3.0-or-later */

/* This linker script was created by merging the information of multipple    */
/* different linker scripts:                                                 */
/* - `riscv64-unknown-elf-ld --verbose`
/* - https://mcyoung.xyz/2021/06/01/linker-script                            */
/* - https://github.com/twilco/riscv-from-scratch                            */
/* - https://github.com/sgmarz/osblog                                        */
/* - https://github.com/rust-embedded/riscv-rt/blob/master/link-rv64.x       */

/* This script needs to be synchronized with                                 */
/* https://github.com/rust-embedded/riscv-rt/blob/28b916dc400caef2b3bfd4c5e66130a162e21f26/link-rv64.x */
/* and updates to this commit need to be synchronized with the updates       */
/* the script gets on GitHub.                                                */

/* We define that the architecture we are building for is RISC-V.            */
/* The value we use if valid for the 32bit and 64bit versions.               */
OUTPUT_ARCH(elf64-littleriscv)
OUTPUT_FORMAT(elf64-littleriscv)

/* The linker can search for archive libraries and ld control scripts here.  */
SEARCH_DIR(/usr/lib/riscv64-unknown-elf/lib)

/* To see the memory map's layout, visit                                     */
/* https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c                  */
/* -                                                                         */
/* There might be other boot ROMs at different addresses, but                */
/* their job is to get to this point.                                        */
MEMORY
{
  /* TODO */
  FLASH (rx) : ORIGIN = 0x20000000, LENGTH = 16M
  /* The RAM region is defined to be read-write ('rw'), executable           */
  /* ('x'), and allocatable ('a'). The RAM memory starts at adress           */
  /* '0x8000_0000'. Technically, the size is arbitrary, and only             */
  /* bounded by the parameter that QEMU received for '-m'. By                */
  /* specifying the size, the linker can double-check that                   */
  /* everything fits.                                                        */
  RAM (rwxa) : ORIGIN = 0x80200000, LENGTH = 16M
}

/* TODO */
REGION_ALIAS("REGION_TEXT",   FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA",   RAM);
REGION_ALIAS("REGION_BSS",    RAM);
REGION_ALIAS("REGION_HEAP",   RAM);
REGION_ALIAS("REGION_STACK",  RAM);

/* TODO */
PROVIDE(_max_hart_id = 0);
/* TODO */
PROVIDE(_hart_stack_size = 2K);
/* TODO */
PROVIDE(_heap_size = 2K);

/* Pre-initialization function. If the user overrides this using the         */
/* `#[pre_init]` attribute or by creating a `__pre_init` function, then the  */
/* function this points to will be called before the RAM is initialized.     */
PROVIDE(__pre_init = default_pre_init);

/* A PAC/HAL defined routine that should initialize custom interrupt         */
/* controller if needed.                                                     */
PROVIDE(_setup_interrupts = default_setup_interrupts);

/* Multi-processing hook function: `fn _mp_hook() -> bool;`.                 */
/* This function is called from all the harts and must return true only for  */
/* one hart,one hart, which will perform memory initialization. For other    */
/* harts it must return false and implement wake-up in platform-dependent    */
/* way (e.g. after waiting for a user interrupt).                            */
PROVIDE(_mp_hook = default_mp_hook);

/* Start trap function override. By default uses the RISC-V crates default   */
/* trap handler but by providing the `_start_trap` symbol external crates    */
/* can override.                                                             */
PROVIDE(_start_trap = default_start_trap);

PROVIDE(UserSoft = DefaultHandler);
PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(UserTimer = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(UserExternal = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

PROVIDE(DefaultHandler = DefaultInterruptHandler);
PROVIDE(ExceptionHandler = DefaultExceptionHandler);

SECTIONS
{
  PROVIDE(_stext = ORIGIN(REGION_TEXT) + SIZEOF_HEADERS);
  . = ABSOLUTE(_stext);

  .text          _stext :
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(SORT_NONE(.init)));
    KEEP(*(SORT_NONE(.init.rust)));
    . = ALIGN(4);
    *(.trap);
    *(.trap.rust);
    *(.text.abort);
    *(.text .text.*);
  } > REGION_TEXT

  .rodata : ALIGN(4)
  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
  } > REGION_RODATA

  .rodata               : ALIGN(4) {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section. This is required by LLD   */
    /* to ensure the LMA of the following .data section will have the        */
    /* correct alignment.                                                    */
    . = ALIGN(4);
  } >REGION_RODATA

  .data : ALIGN(4)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;

    /* Must be called __global_pointer$ for linker relaxations to work.     */
    __global_pointer$ = MIN(_sdata + 0x800, MAX(_sdata + 0x800, _ebss - 0x800));

    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } >REGION_DATA AT>REGION_RODATA

  .bss         (NOLOAD) : {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } >REGION_BSS

  /* "Fictitious" region that represents the memory available for the heap.  */
  .heap        (NOLOAD) : {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } >REGION_HEAP

  /* "Fictitious" region that represents the memory available for the stack. */
  /* Note that `_stack_start` denotes the end of RAM, and our stack grows,   */
  /* just like the RISC-V calling convention demands, from a higher to a     */
  /* lower address.                                                          */
  .stack       (NOLOAD) : {
    _estack = .;
    PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
    . = ABSOLUTE(_stack_start);
    _sstack = .;
  } >REGION_STACK


  /* "Fake" output .got section.                                             */
  /* Dynamic relocations are unsupported. This section is only used to       */
  /* detect relocatable code in the input files and raise an error if        */
  /* relocatable code is found.                                              */
  .got           (INFO) : { KEEP(*(.got .got.*)); }

  /* TODO */
  .eh_frame      (INFO) : { KEEP(*(.eh_frame))    }
  .eh_frame_hdr  (INFO) : { *(.eh_frame_hdr)      }

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

/* Last but not least, we perform some assertions.                           */
/* Do not exceed this mark in the error messages above                                    | */
ASSERT(
  ORIGIN(REGION_TEXT) % 4 == 0, "
  linker-error: the start of the REGION_TEXT must be 4-byte aligned"
);

ASSERT(
  ORIGIN(REGION_RODATA) % 4 == 0, "
  linker-error: the start of the REGION_RODATA must be 4-byte aligned"
);

ASSERT(
  ORIGIN(REGION_DATA) % 4 == 0, "
  linker-error: the start of the REGION_DATA must be 4-byte aligned"
);

ASSERT(
  ORIGIN(REGION_HEAP) % 4 == 0, "
  linker-error: the start of the REGION_HEAP must be 4-byte aligned"
);

ASSERT(
  ORIGIN(REGION_TEXT) % 4 == 0, "
  linker-error: the start of the REGION_TEXT must be 4-byte aligned"
);

ASSERT(
  ORIGIN(REGION_STACK) % 4 == 0, "
  linker-error: the start of the REGION_STACK must be 4-byte aligned"
);

ASSERT(
  _stext % 4 == 0, "
  linker-error: `_stext` must be 4-byte aligned"
);

ASSERT(
  _sdata % 4 == 0 && _edata % 4 == 0, "
  linker-error:  .data is not 4-byte aligned"
);

ASSERT(
  _sidata % 4 == 0, "
  linker-error:  the LMA of .data is not 4-byte aligned"
);

ASSERT(
  _sbss % 4 == 0 && _ebss % 4 == 0, "
  linker-error:  .bss is not 4-byte aligned"
);

ASSERT(
  _sheap % 4 == 0, "
  linker-error:  start of .heap is not 4-byte aligned"
);

ASSERT(
  _stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
  linker-error: The .text section must be placed inside the REGION_TEXT region.
  Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'"
);

ASSERT(
  SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size, "
  linker-error: .stack section is too small for allocating stacks for all the harts.
  Consider changing `_max_hart_id` or `_hart_stack_size`."
);

ASSERT(
  SIZEOF(.got) == 0, "
  linker-error: .got section detected in the input files. Dynamic relocations are
  not supported. If you are linking to C code compiled using the `gcc` crate
  then modify your build script to compile the C code _without_ the
  -fPIC flag. See the documentation of the `gcc::Config.fpic` method for details."
);
/* Do not exceed this mark in the error messages above                                    | */
