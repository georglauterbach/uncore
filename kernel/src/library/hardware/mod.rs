/// ## Architectural Differences
///
/// This module contains architecture specific initialization code and
/// uses conditional compilation.
mod architectures;

pub use architectures::memory;

pub use architectures::init;
