set mi-async
set architecture riscv
set pagination off

file code/target/riscv64gc-unknown-none-elf/debug/uncore
symbol-file code/target/riscv64gc-unknown-none-elf/debug/uncore

br _main
