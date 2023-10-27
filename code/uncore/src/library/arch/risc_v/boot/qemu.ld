/* SPDX-License-Identifier: GPL-3.0-or-later */

/*
  To get an overview of linker scripts, visit
  https://mcyoung.xyz/2021/06/01/linker-script/
*/

/*
  We define that the architecture we are building for is RISC-V.
  The value we use if valid for the 32bit and 64bit versions.
*/
OUTPUT_ARCH(elf64-littleriscv)
OUTPUT_FORMAT(elf64-littleriscv)

/*
  We define the entrypoint, which is the _start function declared
  in Assembly in init.S.
*/
ENTRY(__function__start)

/*
  To see the memory map's layout, visit
  https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c

  There might be other boot ROMs at different addresses, but
  their job is to get to this point.
*/
MEMORY
{
  /*
    The RAM region is defined to be read-write ('rw'), executable
    ('x'), and allocatable ('a'). The memory starts at adress
    '0x8000_0000'. Technically, the size is arbitrary, and only
    bounded by the parameter that QEMU received for '-m'. By
    specifying the size, the linker can double-check that
    everything fits.
  */
  ram  (rwxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/*
  Program headers (PHDRS)

  - text -> CPU instructions (executable sections)
  - data -> Global, initialized variables
  - bss  -> Global, uninitialized variables

  The command PT_LOAD tells the linker that these sections will
  be loaded from the file into memory.

  We could actually stuff all of these into a single program
  header, but by splitting it up into three, we could actually
  use the other PT_* commands such as PT_DYNAMIC, PT_INTERP,
  PT_NULL to tell the linker where to find additional
  information.
*/
PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss  PT_LOAD;
}

SECTIONS
{
  /*
    The first part of our RAM layout will be the text section.
    Since our CPU instructions are here, and our memory starts at
    0x8000_0000, we need our entry point to line up here.
  */
  .text : {
    PROVIDE_HIDDEN(__section__text_start = .);
    /*
      We are going to layout all text sections here, starting with
      .text.init.

      Inside the parentheses is the name of the inout sections. We
      created our own input section called .text.init to make 100%
      sure that the entrypoint is put right at the beginning.

      .text.* means to match anything after .text. If we didn't already specify
      .text.init, this would've matched here. The assembler and linker can place
      things in "special" text sections, so we match any we might come across here.
    */
    *(.text.init)
    *(.text)
    *(.text.*)
    PROVIDE_HIDDEN(__section__text_end = .);

    /*
      To explain what happens here, see
      https://mcyoung.xyz/2021/06/01/linker-script/#memory-regions-and-section-allocation

      C.f. 'MEMORY' above.

      :text tells the linker script to put this into
      the :text program header.
    */
  } >ram AT>ram :text

  /*
    Most compilers create a rodata (read only data) section for
    global constants. However, we're going to place ours in the
    text section. We can actually put this in :data, but since
    the .text section is read-only, we can place it there.

    NOTE: This doesn't actually do anything, yet. The actual
    "protection" cannot be done at link time. Instead, when we
    program the memory management unit (MMU), we will be able
    to choose which bits (R=read, W=write, X=execute) we want
    each memory segment to be able to do.
  */
  .rodata : {
    PROVIDE_HIDDEN(__section__rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE_HIDDEN(__section__rodata_end = .);
  } >ram AT>ram :text

  .data : ALIGN(4096) {
    /*
      sdata and data are essentially the same thing. However,
      compilers usually use the sdata sections for shorter,
      quicker loading sections. So, usually critical data is
      loaded there. However, we're loading all of this in one
      fell swoop. So, we're looking to put all of the following
      sections under the umbrella .data.
    */
    . = ALIGN(4096);
    PROVIDE_HIDDEN(__section__sdata_begin = .);
    *(.sdata .sdata.*)
    PROVIDE_HIDDEN(__section__sdata_end = .);

    PROVIDE_HIDDEN(__section__data_start = .);
    . = ALIGN(4096);
    *(.data .data.*)

    /*
      The global pointer allows the linker to position global
      variables and constants into independent positions relative
      to the gp (global pointer) register. The globals start after
      the text sections and are only relevant to the rodata, data,
      and bss sections.
    */
    __addr__global_pointer = MIN(
      __section__sdata_begin + 0x800,
      MAX(__section__data_start + 0x800, __section__bss_end - 0x800)
    );

    PROVIDE_HIDDEN(__section__data_end = .);
  } >ram AT>ram :data

  .bss : {
    PROVIDE_HIDDEN(__section__bss_start = .);
    *(.sbss .sbss.* .dynsbss)
    *(.bss .bss.* .dynbss)
    PROVIDE_HIDDEN(__section__bss_end = .);
  } >ram AT>ram :bss

  /*
    The following will be helpful when we allocate the kernel
    stack (_stack) and determine where the heap begins and ends
    (__section__heap_start and _heap_start + _heap_size). When we do memory
    allocation, we can use these symbols.

    We use the symbols instead of hard-coding an address because
    this is a floating target. As we add code, the heap moves
    farther down the memory and gets shorter.

    __section___section__memory_start will be set to 0x8000_0000 here. We use
    ORIGIN(ram) so that it will take whatever we set the origin
    of ram to. Otherwise, we'd have to change it more than once
    if we ever stray away from 0x8000_0000 as our entry point.
  */
  PROVIDE_HIDDEN(__addr__memory_start = ORIGIN(ram));
  PROVIDE_HIDDEN(__addr__memory_end = ORIGIN(ram) + LENGTH(ram));

  /*
    Our kernel stack starts at the end of the bss segment
    (__section__section___bss_end). However, we're allocating 0x80000 bytes
    (524 KiB) to our kernel stack. This should be plenty of
    space. The reason we add the memory is because the stack
    grows from higher memory to lower memory (bottom to top).
    Therefore we set the stack at the very bottom of its
    allocated slot. When we go to allocate from the stack,
    we'll subtract the number of bytes we need.
  */
  . = ALIGN(16);
  PROVIDE_HIDDEN(__addr__stack_start = __section__bss_end);
  PROVIDE_HIDDEN(__addr__stack_end = __addr__stack_start + 0x80000);

  /*
     Finally, our heap starts right after the kernel stack.
     We don't align here because we let the kernel determine
     how it wants to do this.
  */
  PROVIDE_HIDDEN(__addr__heap_start = __addr__stack_end);
  PROVIDE_HIDDEN(__addr__heap_size = __addr__memory_end - __addr__heap_start);
}
