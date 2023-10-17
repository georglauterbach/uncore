fn main() {
  let args: Vec<String> = std::env::args().collect();
  dbg!(args);

  let mut path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  path.push_str("/uncore/src/arch/risc-v/qemu.ld");

  std::env::set_var("RUSTFLAGS", format!("-Clink-arg=-T{}", path));
  println!("RUSTFLAGS = {:?}", std::env::var("RUSTFLAGS"));
  std::process::Command::new(env!("CARGO"))
		.arg("build")
		.arg("--target")
		.arg("riscv64gc-unknown-none-elf")
    .arg("--package")
    .arg("uncore")
    .env("RUSTFLAGS", format!("-Clink-arg=-T{}", path))
		.status()
		.expect("Kernel build command did not produce a proper exit status");

  std::process::Command::new("qemu-system-riscv64")
  .args(&["-machine", "virt", "-cpu", "rv64", "-smp", "4", "-m", "128M", "-nographic", "-serial", "mon:stdio", "-device", "virtio-rng-device", "-device", "virtio-gpu-device", "-device", "virtio-net-device", "-device", "virtio-tablet-device", "-device", "virtio-keyboard-device", "-bios", "none", "-kernel", "/home/georg/documents/git/hub/uncore/kernel_new/target/riscv64gc-unknown-none-elf/debug/uncore"]).status().unwrap();
}
