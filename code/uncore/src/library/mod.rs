/// TODO
pub mod arch;

/// TODO
pub mod log;

/// TODO
#[macro_export]
macro_rules! panic_on_error {
    ($function_name:path) => {{
        if let Err(error) = $function_name() {
          panic!("{}", error);
        }
    }};
}
