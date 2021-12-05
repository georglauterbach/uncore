/// # Kernel Version
///
/// The `VERSION` variable contains the kernel version in the semantic
/// versioning format, the git commit id the kernel was built with and
/// the build date. If `VERSION` was not available during build-time,
/// a default value is provided, namely "testing".
pub const VERSION: Option<&str> = option_env!("VERSION");

/// # Global Initialization
///
/// This function takes care of global initialization, i.e.
///
/// - of hardware (registers, ...)
/// - global state and values.
pub fn init() { hardware::init(); }
