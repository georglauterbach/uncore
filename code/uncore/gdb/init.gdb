set mi-async
set architecture riscv
set pagination off

file code/target/riscv64gc-unknown-none-elf/debug/uncore
symbol-file code/target/riscv64gc-unknown-none-elf/debug/uncore

layout asm
layout regs
focus cmd

br _main

define kq
  #kill
  quit 5
end

target remote :1234
continue
