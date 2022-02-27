// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Binary Arguments with [`clap`]
///
/// Manages the argument structure fot the helper binary.
pub mod arguments;

/// ## Builds `unCORE`
///
/// Contains the functions to build `unCORE` properly.
pub mod build;

/// ## Run `unCORE`
///
/// Contains the functions to run `unCORE` properly.
pub mod run;

/// ## Test `unCORE`
///
/// Contains the functions to test `unCORE` and the other workspace members properly.
pub mod test;
