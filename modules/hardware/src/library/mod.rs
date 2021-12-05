mod x86_64;
#[cfg(target_os = "x86_64")] pub use x86_64::init;

mod x86_32;
#[cfg(target_os = "x86_32")] pub use none::init;

mod none;
#[cfg(target_os = "none")] pub use none::init;
