// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub use virtual_::Address as VirtualAddress;
pub use physical::Address as PhysicalAddress;

/// ### Virtual Addresses
///
/// Contains the implementations for the abstraction of the `VirtualAddress` structure.
mod virtual_
{
	/// ### A Virtual Memory Address
	///
	/// A simple wrapper for a virtual address.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Address(usize);

	impl Address
	{
		/// ### Create a New Virtual Address
		///
		/// Wraps a value into an address.
		pub fn new(address: impl Into<usize>) -> Self { Self(address.into()) }

		/// ### Get the Inner Value
		///
		/// Returns the inner value, i.e. content that is wrapped by this type.
		#[must_use]
		pub const fn inner(&self) -> usize { self.0 }

		/// ### Align an Address Down
		///
		/// Takes the address and aligns it down to the given `chunk_size`.
		///
		/// #### Panics
		///
		/// This function will [`panic!`] if
		///
		/// 1. `chunk_size` is not a power of two
		pub fn align_down(&mut self, chunk_size: usize)
		{
			assert!(
				chunk_size.is_power_of_two(),
				"Address alignment down with a chunk size that is not a power of two is \
				 disallowed"
			);

			self.0 &= !(chunk_size - 1);
		}

		/// ### Align an Address Up
		///
		/// Takes the address and aligns it up to the given `chunk_size`.
		///
		/// #### Panics
		///
		/// This function will [`panic!`] if
		///
		/// 1. `chunk_size` is not a power of two
		pub fn align_up(&mut self, chunk_size: usize)
		{
			assert!(
				chunk_size.is_power_of_two(),
				"Address alignment down with a chunk size that is not a power of two is \
				 disallowed"
			);

			let chunk_size_mask = chunk_size - 1;
			if self.0 & chunk_size_mask != 0 {
				self.0 = (self.0 | chunk_size_mask) + 1;
			}
		}
	}

	impl ::core::ops::Add for Address
	{
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.inner()) }
	}

	impl ::core::ops::Add<usize> for Address
	{
		type Output = Self;

		fn add(self, rhs: usize) -> Self::Output { Self::new(self.0 + rhs) }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::Add<u64> for Address
	{
		type Output = Self;

		#[allow(clippy::cast_possible_truncation)]
		fn add(self, rhs: u64) -> Self::Output { Self::new(self.0 + rhs as usize) }
	}

	impl ::core::ops::Add<u32> for Address
	{
		type Output = Self;

		fn add(self, rhs: u32) -> Self::Output { Self::new(self.0 + rhs as usize) }
	}

	impl ::core::ops::Sub for Address
	{
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.inner()) }
	}

	impl ::core::ops::Sub<usize> for Address
	{
		type Output = Self;

		fn sub(self, rhs: usize) -> Self::Output { Self::new(self.0 - rhs) }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::Sub<u64> for Address
	{
		type Output = Self;

		#[allow(clippy::cast_possible_truncation)]
		fn sub(self, rhs: u64) -> Self::Output { Self::new(self.0 - rhs as usize) }
	}

	impl ::core::ops::Sub<u32> for Address
	{
		type Output = Self;

		fn sub(self, rhs: u32) -> Self::Output { Self::new(self.0 - rhs as usize) }
	}

	impl ::core::ops::AddAssign for Address
	{
		fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
	}

	impl ::core::ops::AddAssign<usize> for Address
	{
		fn add_assign(&mut self, rhs: usize) { self.0 += rhs; }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::AddAssign<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn add_assign(&mut self, rhs: u64) { self.0 += rhs as usize; }
	}

	impl ::core::ops::AddAssign<u32> for Address
	{
		fn add_assign(&mut self, rhs: u32) { self.0 += rhs as usize; }
	}

	impl ::core::ops::SubAssign for Address
	{
		fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
	}

	impl ::core::ops::SubAssign<usize> for Address
	{
		fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs; }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::SubAssign<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn sub_assign(&mut self, rhs: u64) { self.0 -= rhs as usize; }
	}

	impl ::core::ops::SubAssign<u32> for Address
	{
		fn sub_assign(&mut self, rhs: u32) { self.0 -= rhs as usize; }
	}

	impl From<usize> for Address
	{
		fn from(address_value: usize) -> Self { Self::new(address_value) }
	}

	#[cfg(target_pointer_width = "64")]
	impl From<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
	}

	impl From<u32> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address_value: u32) -> Self { Self::new(address_value as usize) }
	}

	impl From<Address> for usize
	{
		fn from(address: Address) -> Self { address.0 }
	}

	#[cfg(target_pointer_width = "64")]
	impl From<Address> for u64
	{
		fn from(address: Address) -> Self { address.0 as Self }
	}

	#[cfg(target_pointer_width = "32")]
	impl From<Address> for u32
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address: Address) -> Self { address.0 as Self }
	}
}

/// ### Physical Addresses
///
/// Contains the implementations for the abstraction of the `PhysicalAddress` structure.
mod physical
{
	/// ### A Virtual Memory Address
	///
	/// A simple wrapper for a physical address.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

	pub struct Address(usize);

	impl Address
	{
		/// ### Create a New Physical Address
		///
		/// Constructs a new physical address.
		pub fn new(address: impl Into<usize>) -> Self { Self(address.into()) }

		/// ### Get the Inner Value
		///
		/// Returns the inner value, i.e. content that is wrapped by this type.
		#[must_use]
		pub const fn inner(&self) -> usize { self.0 }

		/// ### Align an Address Down
		///
		/// Takes the address and aligns it down to the given `chunk_size`.
		///
		/// #### Panics
		///
		/// This function will [`panic!`] if
		///
		/// 1. `chunk_size` is not a power of two
		pub fn align_down(&mut self, chunk_size: usize)
		{
			assert!(
				chunk_size.is_power_of_two(),
				"Address alignment down with a chunk size that is not a power of two is \
				 disallowed"
			);

			self.0 &= !(chunk_size - 1);
		}

		/// ### Align an Address Up
		///
		/// Takes the address and aligns it up to the given `chunk_size`.
		///
		/// #### Panics
		///
		/// This function will [`panic!`] if
		///
		/// 1. `chunk_size` is not a power of two
		pub fn align_up(&mut self, chunk_size: usize)
		{
			assert!(
				chunk_size.is_power_of_two(),
				"Address alignment down with a chunk size that is not a power of two is \
				 disallowed"
			);

			let chunk_size_mask = chunk_size - 1;
			if self.0 & chunk_size_mask != 0 {
				self.0 = (self.0 | chunk_size_mask) + 1;
			}
		}
	}

	impl ::core::ops::Add for Address
	{
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.inner()) }
	}

	impl ::core::ops::Add<usize> for Address
	{
		type Output = Self;

		fn add(self, rhs: usize) -> Self::Output { Self::new(self.0 + rhs) }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::Add<u64> for Address
	{
		type Output = Self;

		#[allow(clippy::cast_possible_truncation)]
		fn add(self, rhs: u64) -> Self::Output { Self::new(self.0 + rhs as usize) }
	}

	impl ::core::ops::Add<u32> for Address
	{
		type Output = Self;

		fn add(self, rhs: u32) -> Self::Output { Self::new(self.0 + rhs as usize) }
	}

	impl ::core::ops::Sub for Address
	{
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.inner()) }
	}

	impl ::core::ops::Sub<usize> for Address
	{
		type Output = Self;

		fn sub(self, rhs: usize) -> Self::Output { Self::new(self.0 - rhs) }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::Sub<u64> for Address
	{
		type Output = Self;

		#[allow(clippy::cast_possible_truncation)]
		fn sub(self, rhs: u64) -> Self::Output { Self::new(self.0 - rhs as usize) }
	}

	impl ::core::ops::Sub<u32> for Address
	{
		type Output = Self;

		fn sub(self, rhs: u32) -> Self::Output { Self::new(self.0 - rhs as usize) }
	}

	impl ::core::ops::AddAssign for Address
	{
		fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
	}

	impl ::core::ops::AddAssign<usize> for Address
	{
		fn add_assign(&mut self, rhs: usize) { self.0 += rhs; }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::AddAssign<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn add_assign(&mut self, rhs: u64) { self.0 += rhs as usize; }
	}

	impl ::core::ops::AddAssign<u32> for Address
	{
		fn add_assign(&mut self, rhs: u32) { self.0 += rhs as usize; }
	}

	impl ::core::ops::SubAssign for Address
	{
		fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
	}

	impl ::core::ops::SubAssign<usize> for Address
	{
		fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs; }
	}

	#[cfg(target_pointer_width = "64")]
	impl ::core::ops::SubAssign<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn sub_assign(&mut self, rhs: u64) { self.0 -= rhs as usize; }
	}

	impl ::core::ops::SubAssign<u32> for Address
	{
		fn sub_assign(&mut self, rhs: u32) { self.0 -= rhs as usize; }
	}

	impl From<usize> for Address
	{
		fn from(address_value: usize) -> Self { Self::new(address_value) }
	}

	#[cfg(target_pointer_width = "64")]
	impl From<u64> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address_value: u64) -> Self { Self::new(address_value as usize) }
	}

	impl From<u32> for Address
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address_value: u32) -> Self { Self::new(address_value as usize) }
	}

	impl From<Address> for usize
	{
		fn from(address: Address) -> Self { address.0 }
	}

	#[cfg(target_pointer_width = "64")]
	impl From<Address> for u64
	{
		fn from(address: Address) -> Self { address.0 as Self }
	}

	#[cfg(target_pointer_width = "32")]
	impl From<Address> for u32
	{
		#[allow(clippy::cast_possible_truncation)]
		fn from(address: Address) -> Self { address.0 as Self }
	}
}
