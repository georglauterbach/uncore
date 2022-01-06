// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Miscellaneous Helpers
///
/// Provides various of the most generic helper functions, such as
/// `never_return()`.
pub mod miscellaneous;

/// ## Provides the API for Panicking
///
/// This module provides the implementation for the panic macro and
/// panic behavior.
pub mod panic;

/// ## Providing Support for Tests
///
/// This module provides the implementation to run tests. This
/// includes unit-tests as well as integration tests.
pub mod test;
