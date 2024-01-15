/* SPDX-License-Identifier: GPL-3.0-or-later */

/* This script needs to be synchronized with                                 */
/* https://github.com/rust-embedded/riscv/blob/master/riscv-rt/link.x.in     */
/* and updates to the riscv-rt crate need to be synchronized with            */
/* the updates the script gets on GitHub.                                    */

/* This linker script heavily benefitted from the article by Miguel          */
/* Youn de la Sota: https://mcyoung.xyz/2021/06/01/linker-script/            */

/* We define the architecture we are building for to be RISC-V.              */
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
  /* This is a read-only section of memory that is used to place the text    */
  /* and read-only data into it.                                             */
  REGION_FLASH  (rx) : ORIGIN = 0x20000000, LENGTH = 64M
  /* The DRAM region is defined to be read-write ('rw'), executable          */
  /* ('x'), and allocatable ('a'). The RAM memory starts at adress           */
  /* '0x8000_0000'. Technically, the size is arbitrary, and only             */
  /* bounded by the parameter that QEMU received for '-m'. By                */
  /* specifying the size, the linker can double-check that                   */
  /* everything fits.                                                        */
  REGION_DRAM (rwxa) : ORIGIN = 0x80200000, LENGTH = 16M
}

/* These aliases can be used instead of the memory regions defined in        */
/* MEMORY. These aliases are sued to relocate sections.                      */
REGION_ALIAS(REGION_TEXT,   REGION_FLASH);
REGION_ALIAS(REGION_RODATA, REGION_FLASH);
REGION_ALIAS(REGION_DATA,   REGION_DRAM);
REGION_ALIAS(REGION_BSS,    REGION_DRAM);
REGION_ALIAS(REGION_HEAP,   REGION_DRAM);
REGION_ALIAS(REGION_STACK,  REGION_DRAM);

/* Maximum number of supported hardware threads and their stack size.        */
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);

/* Provide default handlers for possible interrupts and exceptions .         */
PROVIDE(InstructionMisaligned = ExceptionHandler);
PROVIDE(InstructionFault = ExceptionHandler);
PROVIDE(IllegalInstruction = ExceptionHandler);
PROVIDE(Breakpoint = ExceptionHandler);
PROVIDE(LoadMisaligned = ExceptionHandler);
PROVIDE(LoadFault = ExceptionHandler);
PROVIDE(StoreMisaligned = ExceptionHandler);
PROVIDE(StoreFault = ExceptionHandler);;
PROVIDE(UserEnvCall = ExceptionHandler);
PROVIDE(SupervisorEnvCall = ExceptionHandler);
PROVIDE(MachineEnvCall = ExceptionHandler);
PROVIDE(InstructionPageFault = ExceptionHandler);
PROVIDE(LoadPageFault = ExceptionHandler);
PROVIDE(StorePageFault = ExceptionHandler);

PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

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

SECTIONS
{
  .text                 :
  {
    /* Put reset handler first in .text section so it ends up as the entry   */
    /* point of the program. */
    KEEP(*(SORT_NONE(.init)));
    KEEP(*(SORT_NONE(.init.rust)));

    . = ALIGN(4);
    *(.trap);
    *(.trap.rust);
    *(.text.abort);
    *(.text .text.*);
  } >REGION_TEXT

  .rodata               : ALIGN(4)
  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section. This is required by LLD   */
    /* to ensure the LMA of the following .data section will have the        */
    /* correct alignment.                                                    */
    . = ALIGN(4);
  } >REGION_RODATA

  .data                 : ALIGN(4)
  {
    _sidata = LOADADDR(.data); /* required by riscv-rt */
    _sdata = .;                /* required by riscv-rt */

    /* Must be called __global_pointer$ for linker relaxations to work.      */
    __global_pointer$ = MIN(_sdata + 0x800, MAX(_sdata + 0x800, _ebss - 0x800));

    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;                /* required by riscv-rt */
  } >REGION_DATA AT>REGION_RODATA

  /* "Fictitious" region that represents the memory available for            */
  /* zero-initialized data.                                                  */
  .bss         (NOLOAD) :
  {
    _sbss = .;                 /* required by riscv-rt */
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;                 /* required by riscv-rt */
  } >REGION_BSS

  /* "Fictitious" region that represents the memory available for the heap.  */
  .heap        (NOLOAD) : ALIGN(4)
  {
    __heap__start = .;
    __heap__size = 2K;
    . += __heap__size;
    . = ALIGN(4);
  } >REGION_HEAP

  /* "Fictitious" region that represents the memory available for the stack. */
  /* Note that `_stack_start` denotes the end of RAM, and our stack grows,   */
  /* just like the RISC-V calling convention demands, from a higher to a     */
  /* lower address.                                                          */
  .stack       (NOLOAD) : ALIGN(16) {
    /* required by riscv-rt */
    _stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK);
    . = ABSOLUTE(_stack_start);
  } >REGION_STACK

  /* "Fake" output .got section.                                             */
  /* Dynamic relocations are unsupported. This section is only used to       */
  /* detect relocatable code in the input files and raise an error if        */
  /* relocatable code is found.                                              */
  .got           (INFO) : { KEEP(*(.got .got.*)); }

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

  /DISCARD/             : { *(.note.*) *(.gnu.*) *(.eh_frame) *(.eh_frame_hdr) }
}

/* Last but not least, we perform assertions.                                */

ASSERT(ORIGIN(REGION_DRAM)  % 4 == 0, "The memory region REGION_RAM is not 4-byte aligned");
ASSERT(ORIGIN(REGION_FLASH) % 4 == 0, "The memory region REGION_FLASH is not 4-byte aligned");

ASSERT(ADDR(.text) % 4 == 0,               "The section .text is not 4-byte aligned");
ASSERT(_sdata % 4 == 0 && _edata % 4 == 0, "The section .data is not 4-byte aligned");
ASSERT(_sidata % 4 == 0,                   "The LMA of the section .data is not 4-byte aligned");
ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0,   "The section .bss is not 4-byte aligned");
ASSERT(ADDR(.heap) % 4 == 0,               "The section .heap is not 4-byte aligned");

ASSERT(
  ADDR(.text) + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT),
  "The .text section does not seem to fit inside the REGION_TEXT region."
);

ASSERT(
  SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size,
  ".stack section is too small for allocating stacks for all the harts.
  Consider changing `_max_hart_id` or `_hart_stack_size`."
);

ASSERT(
  SIZEOF(.got) == 0,
  ".got section detected in the input files. Dynamic relocations are
  not supported. If you are linking to C code compiled using the `gcc` crate
  then modify your build script to compile the C code _without_ the
  -fPIC flag. See the documentation of the `gcc::Config.fpic` method for details."
);
