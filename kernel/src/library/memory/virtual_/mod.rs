// SPDX-License-Identifier: GPL-3.0-or-later

/// ### Address Types for Virtual Memory
///
/// This module contains abstractions for virtual and physical addresses - both needed for
/// a clear and clean abstraction when using virtual memory.
pub mod addresses;

/// ### "Region Sizes" for Virtual Memory
///
/// This module contains types that indicates the size of pages or frames.
pub mod chunks;

/// ## Demand Paging
///
/// Contains the needed types for proper demand paging.
pub mod paging;

/// ### Kernel Page Table
///
/// Represents the global page table held by the kernel for demand paging.
pub(super) static mut KERNEL_PAGE_TABLE: spin::Mutex<
	spin::once::Once<crate::prelude::architectures::memory::PageTable>,
> = spin::Mutex::new(spin::Once::new());
