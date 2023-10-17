// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;

/// ### Setup and Initialize Interrupts
///
/// This function mainly takes care of enabling interrupts.
pub(super) fn setup_and_enable()
{
	log_debug!("Setting up and enabling interrupts");
	log_warning!("Interrupts are currently not implemented");

	// crate::log_debug!("Enabling interrupts");
	// x86_64::instructions::interrupts::enable();
}
