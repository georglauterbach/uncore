// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

pub use virtual_::Address as VirtualAddress;
pub use physical::Address as PhysicalAddress;

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
		/// Takes a
		pub fn new(address: usize) -> Self { Self(address) }

		/// ### Get the Inner Value
		///
		/// Returns the inner value, i.e. content that is wrapped by this type.
		pub fn inner(&self) -> usize { self.0 }
	}

	impl From<usize> for Address
	{
		fn from(address_value: usize) -> Self { Self::new(address_value) }
	}

	impl From<Address> for usize
	{
		fn from(address: Address) -> Self { address.inner() }
	}

	impl ::core::ops::Add for Address
	{
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output { Self::new(self.inner() + rhs.inner()) }
	}

	impl ::core::ops::Add<usize> for Address
	{
		type Output = Self;

		fn add(self, rhs: usize) -> Self::Output { Self::new(self.inner() + rhs) }
	}

	impl ::core::ops::Add<u64> for Address
	{
		type Output = Self;

		fn add(self, rhs: u64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
	}

	impl ::core::ops::Add<i64> for Address
	{
		type Output = Self;

		fn add(self, rhs: i64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
	}

	impl ::core::ops::Sub for Address
	{
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output { Self::new(self.inner() - rhs.inner()) }
	}

	impl ::core::ops::Sub<usize> for Address
	{
		type Output = Self;

		fn sub(self, rhs: usize) -> Self::Output { Self::new(self.inner() - rhs) }
	}

	impl ::core::ops::Sub<u64> for Address
	{
		type Output = Self;

		fn sub(self, rhs: u64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
	}

	impl ::core::ops::Sub<i64> for Address
	{
		type Output = Self;

		fn sub(self, rhs: i64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
	}
}

mod physical
{

	/// ### A Physical Address Abstraction
	///
	/// This is an opaque wrapper type that contains the address as its first type.
	#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
		pub fn inner(&self) -> usize { self.0 }
	}

	impl ::core::ops::Add for Address
	{
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output { Self::new(self.inner() + rhs.inner()) }
	}

	impl ::core::ops::Add<usize> for Address
	{
		type Output = Self;

		fn add(self, rhs: usize) -> Self::Output { Self::new(self.inner() + rhs) }
	}

	impl ::core::ops::Add<u64> for Address
	{
		type Output = Self;

		fn add(self, rhs: u64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
	}

	impl ::core::ops::Add<i64> for Address
	{
		type Output = Self;

		fn add(self, rhs: i64) -> Self::Output { Self::new(self.inner() + rhs as usize) }
	}

	impl ::core::ops::Sub for Address
	{
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output { Self::new(self.inner() - rhs.inner()) }
	}

	impl ::core::ops::Sub<usize> for Address
	{
		type Output = Self;

		fn sub(self, rhs: usize) -> Self::Output { Self::new(self.inner() - rhs) }
	}

	impl ::core::ops::Sub<u64> for Address
	{
		type Output = Self;

		fn sub(self, rhs: u64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
	}

	impl ::core::ops::Sub<i64> for Address
	{
		type Output = Self;

		fn sub(self, rhs: i64) -> Self::Output { Self::new(self.inner() - rhs as usize) }
	}
}
