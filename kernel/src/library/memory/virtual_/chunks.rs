// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### Determine Frame and Page Size
///
/// This struct can is used to abstract over all available page sizes of a system.
pub trait ChunkSize: Copy
{
	/// Page size is bytes.
	const SIZE: usize;

	/// Page size as string for debug purposes.
	const SIZE_AS_DEBUG_STRING: &'static str;
}

/// ### Architecture Default Frame or Page Size
///
/// Represents the default page size for an architecture. On `x86_64` the size is 4096
/// Bytes.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeDefault;

impl ChunkSizeDefault
{
	/// ### Default Size
	///
	/// Returns the size of the default page size ([`ChunkSizeDefault::SIZE`]).
	pub(crate) const fn size() -> usize { Self::SIZE }
}

/// ### Architecture's Big Pages
///
/// The bigger-than-default pages.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeHuge;

/// ### Architecture's Biggest Pages
///
/// This is the biggest page size available for an architecture.
#[derive(Debug, Clone, Copy)]
pub struct ChunkSizeGiant;
