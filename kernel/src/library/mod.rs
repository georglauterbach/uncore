// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Architecture Specific Code
///
/// Holds **all** _architecture dependent_ code. That includes (but
/// not exclusively)
///
/// - assembly boot code
/// - CPU initialization code
/// - virtual memory initialization code
pub mod architectures;

/// ## Generic Helper Function
///
/// Holds many of the generic functions re-exported in the `prelude`
/// module. These include panic callbacks, test infrastructure or
/// information about the kernel.
mod helper;

/// ## Uniform Logging
///
/// This module exports the `log_!` macros with different log levels.
/// It implements the `log` crate's logging facade.
pub mod log;

/// ## Virtual Memory
///
/// This module handles virtual memory, that is (demand) paging,
/// allocations, etc. for the user- and kernel-space.
///
/// ### About Heap Allocations
///
/// Be especially careful in this section of the kernel to not accidentally use the
/// allocator
///
/// 1. before it was initialized or when you cannot be sure whether it is already
///    initialized
/// 2. in contexts where it is inappropriate or where it could lead to
///    allocation loops
pub mod memory;

/// ## The Kernel Prelude
///
/// This module provides common
///
/// - modules
/// - structures
/// - macros
/// - functions
///
/// used by many other modules. It should be imported via
///
/// ``` edition2021
/// use library::prelude::*;
/// ```
pub mod prelude;
