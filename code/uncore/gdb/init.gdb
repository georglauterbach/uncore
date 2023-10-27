set mi-async
set architecture riscv
set pagination off
set print asm-demangle on

file code/target/riscv64gc-unknown-none-elf/debug/uncore
symbol-file code/target/riscv64gc-unknown-none-elf/debug/uncore

layout asm
layout regs
focus cmd

br uncore::__risc_v_rt__main

define kq
  kill
  quit 5
end

target remote :1234
continue
