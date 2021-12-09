mod arch_x86_64;
#[cfg(target_arch = "x86_64")] pub use arch_x86_64::init;

mod arch_x86_32;
#[cfg(target_arch = "x86_32")] pub use arch_x86_32::init;
