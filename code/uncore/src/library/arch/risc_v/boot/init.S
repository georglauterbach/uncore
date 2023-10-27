// SPDX-License-Identifier: GPL-3.0-or-later

# ? INFO: instructions are opportunistically compressed by the RISC-V assembler
# ?       when possible (but sometimes this behavior is not desirable.).
# ! TASK: disable the generation of compressed instructions
# .option norvc

.section .init, "ax"

# ? INFO: _start is the first function that is executed
# ! TASK: setup registers, set up memory & jump to Rust
.global __function__start
__function__start:
  # ? INFO: any hardware threads (hart) that are not bootstrapping need to wait for an IPI
  # * REF:  3.1.5 of "Volume II: RISC-V Privileged Architectures V20211203"
  # ! TASK: read our hart identifier into the register t0 and see if it is 0;
  # !       if it isn't, we send it to be parked (busy loop)
  # -----------------------------------------------------------------------------------------------
  # - mhartid = machine hardware thread ID
  csrr  t0, mhartid   # control status register read: t0 <- mhartid
  bnez  t0, busy_loop # branch not equal zero: t0 != 0 -> jump to busy_loop
  # -----------------------------------------------------------------------------------------------

  # ? INFO: this is how we will eventually control the MMU
  # ! TASK: set satp to 0, disabling virtual memory for now
  # -----------------------------------------------------------------------------------------------
  # - satp = supervisor address translation and protection register
  csrw   satp, zero # control status register write: 0 -> satp
  # -----------------------------------------------------------------------------------------------

  # ? INFO: the global pointer register  allows the linker to position global
  # ?       variables and constants into the gp register
  # * REF:  https://sourceware.org/binutils/docs-2.34/as/RISC_002dV_002dDirectives.html
	# ! TASK: set up the global pointer register
  # -----------------------------------------------------------------------------------------------
  # - pushes the current option stack; used whenever changing an option in line
  #   with assembly code in order to ensure the user’s command-line options are
  #   respected for the bulk of the file being assembled.
  .option push
  # - since we relax addressing sequences to shorter GP-relative sequences when
  #   possible, the initial load of GP must not be relaxed
  .option norelax
  # - gp = global pointer (register)
  la    gp, __global_pointer$ # load address: gp <- _global_pointer
  # - pops the current option stack (see .option push above)
  .option pop
  # -----------------------------------------------------------------------------------------------

  # ? INFO: __section__section___section___section__bss_start & __bss_end are symbols set by the linker script
  # ! TASK: initialize the BSS section, which is expected to be zero
  # -----------------------------------------------------------------------------------------------
#   la    a0, __section__bss_start # load address: a0 <- __bss_start
#   la    a1, __section__bss_end   # load address: a1 <- __bss_end
#   bgeu  a0, a1, 2f     # branch greater or equal unsigned: a0 >= a1 -> jump to 2 (forward)
# 1:
#   sd    zero, (a0)     # store double (64bit): 0 -> address of a0
#   addi  a0, a0, 8      # add immediate: a0 <- a0 + 8
#   bltu  a0, a1, 1b     # branch less that unsigned: a0 < a1 -> jump to 1 (backward)
# 2:
  # -----------------------------------------------------------------------------------------------

  # ! TASK: set up control registers and jump to Rust
  # -----------------------------------------------------------------------------------------------
  # - load kernel stack pointer
  la    sp, __stack_top # load address: sp <- _stack

  # - properly set the mstatus register
  #   - setting bits 12 & 11 to 0b11 (= "machine mode") gives us access to all
  #     instructions and registers
  #   - bit 7 & bit 3 enables interrupts at a coarse level; we will still
  #     need to enable particular interrupts through the mie (machine
  #     interrupt enable) register, which we do later
  li    t0, (0b11 << 11) | (1 << 7) | (1 << 3) # load immediate: t0 <- bitfield
  csrw  mstatus, t0                            # control status register write: t0 -> mstatus

  # - properly set the register that installed into pc when taking an exception
  #   - mepc = machine exception program counter (installed into pc when returning
  #     from an exception)
  #   - _main is defined in Rust (as a function)
  la    t1, __function__main # load address: t1 <- _main
  csrw  mepc, t1  # control status register write: t1 -> mepc

  # - properly set the address that is to be used when a trap (system call,
  #   illegal instruction, timer interrupt, etc.) occurs
  #   - mtvec = machine trap vector
  la    t2, asm_trap_vector # load address: t2 <- asm_trap_vector
  csrw  mtvec, t2           # control status register write: t2 -> mtvec

  # - (follow-up) with properly enabling interrupts
  #   - mie = machine interrupt enable (register)
  li    t3, (1 << 3) | (1 << 7) | (1 << 11) # load immediate: t3 <- bitfield
  # csrw  mie, t3                             # control status register write: t3 -> mie

  # - set a return address after _main has finished running
  #   - ra = return address
  la    ra, busy_loop

  # - re-enable the generation of compressed instructions (see .option norvc
  #   above)
  # .option rvc

  # - "return" from an exception; we use mret at the end so that the mstatus register
  #   is properly updated
  mret
  # -----------------------------------------------------------------------------------------------

# ! TASK: set up a busy-loop that does nothing except waiting for interrupts
busy_loop:
  wfi
  j     busy_loop

.section .text

# ? INFO: we get here when the CPU is interrupted for any reason
# ! TASK: handle the interrupt / fault
.global asm_trap_vector
asm_trap_vector:
  mret # TODO
