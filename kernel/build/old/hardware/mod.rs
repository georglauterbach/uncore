// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Architectural Differences
///
/// This module contains architecture specific initialization code and
/// uses conditional compilation.
mod architectures;

pub use architectures::memory;

pub use architectures::init;
