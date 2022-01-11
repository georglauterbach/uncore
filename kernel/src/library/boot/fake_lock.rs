// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use ::core::cell;

/// ### The Unsafe Lock Itself
///
/// The fake locking structure for the multiboot2 information.
pub struct Lock<T>
{
	/// The only data filed, a generic data type.
	data: cell::UnsafeCell<T>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T> Send for Lock<T> {}
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T>
{
	/// ### Create a New Lock.
	///
	/// This is a constant function and can subsequently
	/// be used in global statics.
	pub const fn new(data: T) -> Self
	{
		Self {
			data: cell::UnsafeCell::new(data),
		}
	}

	/// ### Get A Reference to the Inner Data
	///
	/// Returns a read only reference to the data
	/// encapsulated with `as_ref()`.
	pub const fn get(&self) -> &T { unsafe { &*self.data.get() } }

	/// ### Get a Mutable Reference
	///
	/// Returns mutable reference to the data inside.
	#[allow(clippy::mut_from_ref)]
	pub fn get_mut(&self) -> &mut T { unsafe { &mut *self.data.get() } }
}
