// SPDX-License-Identifier: GPL-3.0-or-later

//! TODO

pub mod uart;

/// TODO
pub(super) fn initialize() {
  // TODO make this function callable only once!
  uart::Uart::init();
}
