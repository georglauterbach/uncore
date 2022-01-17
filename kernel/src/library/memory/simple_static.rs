// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::prelude::*;

use global_static_allocator::GlobalStaticChunkAllocator;
use pages::PageAlignedByteBuf;

/// Chunk size must be a multiple of 8, so that the bitmap can cover
/// all fields properly.
const MULTIPLE_OF: usize = 8;
/// 32768 chunks -> 8 MiB Heap. Must be be a multiple of 8.
pub const HEAP_SIZE: usize = GlobalStaticChunkAllocator::CHUNK_SIZE * MULTIPLE_OF * 4096;
static mut HEAP: PageAlignedByteBuf<HEAP_SIZE> = PageAlignedByteBuf::new_zeroed();
// always make sure, that the division is "clean", i.e. no remainder
const BITMAP_SIZE: usize = HEAP_SIZE / GlobalStaticChunkAllocator::CHUNK_SIZE / 8;
static mut BITMAP: PageAlignedByteBuf<BITMAP_SIZE> = PageAlignedByteBuf::new_zeroed();

#[global_allocator]
static KERNEL_HEAP: GlobalStaticChunkAllocator = GlobalStaticChunkAllocator::new();

/// Initializes the global static rust allocator. It uses static
/// memory already available inside the address space.
pub fn initialize()
{
	log_info!("Initializing a simple, static allocator");
	unsafe {
		KERNEL_HEAP
			.init(HEAP.get_mut(), BITMAP.get_mut())
			.expect("Could not acquire the heap bitmap")
	}
	log_debug!("Initialized allocator");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> !
{
	panic!("alloc error: {:#?}", layout);
}

mod pages
{

	use core::ops::{
		Deref,
		DerefMut,
	};

	pub const PAGE_SIZE: usize = 4096;

	#[repr(align(4096))]
	#[derive(Clone, Debug)]
	pub struct PageAligned<T>(T);

	impl<T> PageAligned<T>
	{
		/// Constructor that takes ownership of the data. The
		/// data is guaranteed to be aligned.
		pub const fn new(t: T) -> Self { Self(t) }

		#[cfg(test)]
		const fn self_ptr(&self) -> *const Self { self as *const _ }

		/// Returns the pointer to the data. The pointer is
		/// the address of a page, because the data is
		/// page-aligned.
		pub const fn data_ptr(&self) -> *const T { (&self.0) as *const _ }

		/// Returns the number of the page inside the address
		/// space.
		pub fn page_num(&self) -> usize { self.data_ptr() as usize / PAGE_SIZE }

		/// Returns the address of this struct. Because this
		/// struct is page-aligned, the address is the address
		/// of a page.
		pub fn page_addr(&self) -> usize
		{
			self.data_ptr() as usize // & !0xfff not relevant because aligned
		}

		/// Returns a reference to the underlying data.
		pub const fn get(&self) -> &T { &self.0 }

		/// Returns a mutable reference to the underlying
		/// data.
		pub const fn get_mut(&mut self) -> &mut T { &mut self.0 }

		/// Consumes the struct and returns the owned, inner
		/// data.
		pub fn into_inner(self) -> T { self.0 }
	}

	impl<T: Copy> Copy for PageAligned<T> {}

	impl<T> From<T> for PageAligned<T>
	{
		fn from(data: T) -> Self { PageAligned::new(data) }
	}

	impl<T> Deref for PageAligned<T>
	{
		type Target = T;

		fn deref(&self) -> &Self::Target { self.get() }
	}

	impl<T> DerefMut for PageAligned<T>
	{
		fn deref_mut(&mut self) -> &mut Self::Target { self.get_mut() }
	}

	/// Convenient wrapper around [`PageAligned`] for aligned
	/// stack-buffers, with exactly the same restrictions and
	/// properties.
	#[repr(align(4096))]
	#[derive(Clone, Debug)]
	pub struct PageAlignedBuf<T, const N: usize>(PageAligned<[T; N]>);

	impl<T: Copy, const N: usize> PageAlignedBuf<T, N>
	{
		/// Constructor that fills the default element into
		/// each index of the slice. Uses this approach in
		/// favor of `Default`, because this works in a const
		/// context.
		pub const fn new(default: T) -> Self { Self(PageAligned::new([default; N])) }
	}

	impl<T, const N: usize> PageAlignedBuf<T, N>
	{
		/// Return a pointer to self.
		pub const fn self_ptr(&self) -> *const Self { self.0.data_ptr() as *const _ }

		/// Returns the number of the page inside the address
		/// space.
		pub fn page_num(&self) -> usize { self.0.page_num() }

		/// Returns the page base address of this struct.
		pub fn page_base_addr(&self) -> usize { self.0.page_addr() }

		/// Returns a reference to the underlying data.
		pub const fn get(&self) -> &[T; N] { self.0.get() }

		/// Returns a reference to the underlying data.
		pub const fn get_mut(&mut self) -> &mut [T; N] { self.0.get_mut() }
	}

	impl<T: Copy, const N: usize> Copy for PageAlignedBuf<T, N> {}

	impl<const N: usize> PageAlignedBuf<u8, N>
	{
		/// New `u8` buffer that is initialized with zeroes.
		pub const fn new_zeroed() -> Self { Self::new(0) }
	}

	impl<T, const N: usize> Deref for PageAlignedBuf<T, N>
	{
		type Target = [T; N];

		fn deref(&self) -> &Self::Target { self.get() }
	}

	impl<T, const N: usize> DerefMut for PageAlignedBuf<T, N>
	{
		fn deref_mut(&mut self) -> &mut Self::Target { self.get_mut() }
	}

	/// Convenient alias for [`PageAlignedBuf`].
	pub type PageAlignedByteBuf<const N: usize> = PageAlignedBuf<u8, N>;
}

mod global_static_allocator
{

	mod chunk_allocator
	{
		//! Module for [`ChunkAllocator`].

		use core::alloc::Layout;

		/// Possible errors for [`ChunkAllocator`].
		/// TODO make more generic ?! later use in root task
		/// and native hedron app with different allocator
		/// frontends?
		#[derive(Debug)]
		pub enum ChunkAllocatorError
		{
			/// The backing memory for the heap must be
			/// - an even multiple of
			///   [`DEFAULT_ALLOCATOR_CHUNK_SIZE`], and
			/// - a multiple of 8 to be correctly
			///   represented by the bitmap.
			BadHeapMemory,
			/// The number of bits in the backing memory
			/// for the heap bitmap must match the number
			/// of chunks in the heap.
			BadBitmapMemory,
		}

		pub const DEFAULT_ALLOCATOR_CHUNK_SIZE: usize = 256;

		/// First-fit allocator that takes mutable references
		/// to arbitrary external memory backing storages. It
		/// uses them to manage memory. It is mandatory to
		/// wrap this allocator by a mutex or a similar
		/// primitive, if it should be used in a global
		/// context. It can take (global) static memory arrays
		/// as backing storage. It allocates memory in chunks
		/// of custom length, i.e. `256` or `4096`.
		/// Default value is [`DEFAULT_ALLOCATOR_CHUNK_SIZE`].
		///
		/// This can be used to construct allocators, that
		/// manage the heap for the roottask or virtual
		/// memory.
		///
		/// TODO: In fact, the chunk allocator only needs the
		/// bitmap reference, but not the  one from the heap.
		/// Future work: completely throw this away and
		/// instead do some  mixture of PAge-Frame-Allocator
		/// and Virtual Memory Mapper
		#[derive(Debug)]
		pub struct ChunkAllocator<'a, const CHUNK_SIZE: usize>
		{
			heap:   &'a mut [u8],
			bitmap: &'a mut [u8],
		}

		impl<'a, const CHUNK_SIZE: usize> ChunkAllocator<'a, CHUNK_SIZE>
		{
			/// Returns the default const generic value of
			/// `CHUNK_SIZE`.
			#[allow(unused)]
			pub const fn default_chunk_size() -> usize
			{
				// keep in sync with struct definition!
				DEFAULT_ALLOCATOR_CHUNK_SIZE
			}

			/// Returns the used chunk size.
			#[allow(unused)]
			pub const fn chunk_size(&self) -> usize { CHUNK_SIZE }

			/// Creates a new allocator object. Verifies
			/// that the provided memory has the correct
			/// properties.
			/// - heap length must be a multiple of
			///   `CHUNK_SIZE`
			/// - the heap must be >= 0
			pub const fn new(
				heap: &'a mut [u8],
				bitmap: &'a mut [u8],
			) -> Result<Self, ChunkAllocatorError>
			{
				let is_empty = heap.len() == 0;
				let is_not_multiple_of_chunk_size = heap.len() % CHUNK_SIZE != 0;
				let is_not_coverable_by_bitmap = heap.len() < 8 * CHUNK_SIZE;
				if is_empty
					|| is_not_multiple_of_chunk_size || is_not_coverable_by_bitmap
				{
					return Err(ChunkAllocatorError::BadHeapMemory);
				}

				// check bitmap memory has correct length
				let expected_bitmap_length = heap.len() / CHUNK_SIZE / 8;
				if bitmap.len() != expected_bitmap_length {
					return Err(ChunkAllocatorError::BadBitmapMemory);
				}

				Ok(Self { heap, bitmap })
			}

			/// Capacity in bytes of the allocator.
			pub const fn capacity(&self) -> usize { self.heap.len() }

			/// Returns number of chunks.
			pub fn chunk_count(&self) -> usize
			{
				// size is a multiple of CHUNK_SIZE;
				// ensured in new()
				self.capacity() / CHUNK_SIZE
			}

			/// Returns whether a chunk is free according
			/// to the bitmap.
			///
			/// # Parameters
			/// - `chunk_index` describes the start chunk;
			///   i.e. the search space inside the backing
			///   storage
			fn chunk_is_free(&self, chunk_index: usize) -> bool
			{
				assert!(chunk_index < self.chunk_count());
				let (byte_i, bit) = self.chunk_index_to_bitmap_indices(chunk_index);
				let relevant_bit = (self.bitmap[byte_i] >> bit) & 1;
				relevant_bit == 0
			}

			/// Marks a chunk as used, i.e. write a 1 into
			/// the bitmap at the right position.
			fn mark_chunk_as_used(&mut self, chunk_index: usize)
			{
				assert!(chunk_index < self.chunk_count());
				if !self.chunk_is_free(chunk_index) {
					panic!(
						"tried to mark chunk {} as used but it is already \
						 used",
						chunk_index
					);
				}
				let (byte_i, bit) = self.chunk_index_to_bitmap_indices(chunk_index);
				// xor => keep all bits, except bitflip at relevant position
				self.bitmap[byte_i] = self.bitmap[byte_i] ^ (1 << bit);
			}

			/// Marks a chunk as free, i.e. write a 0 into
			/// the bitmap at the right position.
			fn mark_chunk_as_free(&mut self, chunk_index: usize)
			{
				assert!(chunk_index < self.chunk_count());
				if self.chunk_is_free(chunk_index) {
					panic!(
						"tried to mark chunk {} as free but it is already \
						 free",
						chunk_index
					);
				}
				let (byte_i, bit) = self.chunk_index_to_bitmap_indices(chunk_index);
				// xor => keep all bits, except bitflip at relevant position
				let updated_byte = self.bitmap[byte_i] ^ (1 << bit);
				self.bitmap[byte_i] = updated_byte;
			}

			/// Returns the indices into the bitmap array
			/// of a given chunk index.
			fn chunk_index_to_bitmap_indices(
				&self,
				chunk_index: usize,
			) -> (usize, usize)
			{
				assert!(
					chunk_index < self.chunk_count(),
					"chunk_index out of range!"
				);
				(chunk_index / 8, chunk_index % 8)
			}

			/// Returns the indices into the bitmap array
			/// of a given chunk index.
			#[allow(unused)]
			fn bitmap_indices_to_chunk_index(
				&self,
				bitmap_index: usize,
				bit: usize,
			) -> usize
			{
				let chunk_index = bitmap_index * 8 + bit;
				assert!(
					chunk_index < self.chunk_count(),
					"chunk_index out of range!"
				);
				chunk_index
			}

			/// Returns the index of the next free chunk
			/// of memory that is correctly aligned.
			///
			/// # Parameters
			/// - `start_chunk` describes the start chunk;
			///   i.e. the search space inside the backing
			///   storage
			/// - `alignment` required alignment of the
			///   chunk in memory
			///
			/// # Return
			/// Returns the index of the chunk or `Err`
			/// for out of memory.
			fn find_next_free_chunk_aligned(
				&self,
				start_chunk: Option<usize>,
				alignment: u32,
			) -> Result<usize, ()>
			{
				let start_chunk = start_chunk.unwrap_or(0);

				if start_chunk >= self.chunk_count() {
					log::debug!("chunk_index out of range!");
					return Err(());
				}

				for i in start_chunk..self.chunk_count() {
					if self.chunk_is_free(i) {
						let addr = unsafe { self.chunk_index_to_ptr(i) }
							as u32;
						let is_aligned = addr % alignment == 0;
						if is_aligned {
							return Ok(i);
						}
					}
				}

				// out of memory
				Err(())
			}

			/// Finds the next available chain of
			/// available chunks. Returns the
			/// beginning index.
			///
			/// # Parameters
			/// - `chunk_num` number of chunks that must
			///   be all free without gap in-between;
			///   greater than 0
			/// - `alignment` required alignment of the
			///   chunk in memory
			fn find_free_coherent_chunks_aligned(
				&self,
				chunk_num: usize,
				alignment: u32,
			) -> Result<usize, ()>
			{
				assert!(
					chunk_num > 0,
					"chunk_num must be greater than 0! Allocating 0 blocks \
					 makes no sense"
				);
				let mut begin_chunk_i =
					self.find_next_free_chunk_aligned(Some(0), alignment)?;
				let out_of_mem_cond =
					begin_chunk_i + (chunk_num - 1) >= self.chunk_count();
				while !out_of_mem_cond {
					// this var counts how many coherent chunks we found while
					// iterating the bitmap
					let mut coherent_chunk_count = 1;
					for chunk_chain_i in 1..=chunk_num {
						if coherent_chunk_count == chunk_num {
							return Ok(begin_chunk_i);
						} else if self.chunk_is_free(
							begin_chunk_i + chunk_chain_i,
						) {
							coherent_chunk_count += 1;
						} else {
							break;
						}
					}

					// check again at next free block
					// "+1" because we want to skip the just discovered non-free
					// block
					begin_chunk_i = self
						.find_next_free_chunk_aligned(
							Some(begin_chunk_i
								+ coherent_chunk_count + 1),
							alignment,
						)
						.unwrap();
				}
				// out of memory
				Err(())
			}

			/// Returns the pointer to the beginning of
			/// the chunk.
			unsafe fn chunk_index_to_ptr(&self, chunk_index: usize) -> *mut u8
			{
				assert!(
					chunk_index < self.chunk_count(),
					"chunk_index out of range!"
				);
				self.heap.as_ptr().add(chunk_index * CHUNK_SIZE) as *mut u8
			}

			/// Returns the chunk index of the given
			/// pointer (which points to the beginning of
			/// a chunk).
			unsafe fn ptr_to_chunk_index(&self, ptr: *const u8) -> usize
			{
				let heap_begin_inclusive = self.heap.as_ptr();
				let heap_end_exclusive = self.heap.as_ptr().add(self.heap.len());
				assert!(
					heap_begin_inclusive <= ptr && ptr < heap_end_exclusive,
					"pointer {:?} is out of range {:?}..{:?} of the \
					 allocators backing storage",
					ptr,
					heap_begin_inclusive,
					heap_end_exclusive
				);
				(ptr as usize - heap_begin_inclusive as usize) / CHUNK_SIZE
			}

			#[track_caller]
			pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8
			{
				let layout = if layout.size() == 0 {
					Layout::from_size_align(1, layout.align()).unwrap()
				} else {
					layout
				};

				let mut required_chunks = layout.size() / CHUNK_SIZE;
				let modulo = layout.size() % CHUNK_SIZE;

				// log::debug!("alloc: layout={:?} ({} chunks]", layout,
				// required_chunks);

				if modulo != 0 {
					required_chunks += 1;
				}

				let index = self.find_free_coherent_chunks_aligned(
					required_chunks,
					layout.align() as u32,
				);

				if let Err(_) = index {
					panic!(
						"Out of Memory. Can't fulfill the requested \
						 layout: {:?}",
						layout,
					);
				}
				let index = index.unwrap();

				for i in index..index + required_chunks {
					self.mark_chunk_as_used(i);
				}

				self.chunk_index_to_ptr(index)
			}

			#[track_caller]
			pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout)
			{
				let mut required_chunks = layout.size() / CHUNK_SIZE;
				let modulo = layout.size() % CHUNK_SIZE;
				if modulo != 0 {
					required_chunks += 1;
				}
				// log::debug!("dealloc: layout={:?} ({} chunks]", layout,
				// required_chunks);

				let index = self.ptr_to_chunk_index(ptr as *const u8);
				for i in index..index + required_chunks {
					self.mark_chunk_as_free(i);
				}
			}
		}
	}

	use chunk_allocator::{
		ChunkAllocator,
		ChunkAllocatorError,
		DEFAULT_ALLOCATOR_CHUNK_SIZE,
	};
	use spin::Mutex as SimpleMutex;
	use core::alloc::{
		GlobalAlloc,
		Layout,
	};

	#[derive(Debug)]
	pub enum GlobalStaticChunkAllocatorError
	{
		Uninitialized,
		AlreadyInitialized,
		/// Error in the inner allocator object.
		Inner(ChunkAllocatorError),
	}

	/// Wrapping struct around [`ChunkAllocator`] which enables
	/// the usage of this allocator in a global context, i.e. as
	/// global allocator. Memory is allocated in blocks/chunks
	/// with a size of [`GlobalStaticChunkAllocator::CHUNK_SIZE`].
	///
	/// The struct synchronized accesses to the underlying memory.
	/// It must be initialized by calling [`Self::init`],
	/// otherwise allocations result in panics.
	#[derive(Debug)]
	pub struct GlobalStaticChunkAllocator<'a>
	{
		inner_allocator:
			SimpleMutex<Option<ChunkAllocator<'a, DEFAULT_ALLOCATOR_CHUNK_SIZE>>>,
	}

	impl<'a> GlobalStaticChunkAllocator<'a>
	{
		/// Publicly make the default chunk size available.
		pub const CHUNK_SIZE: usize = DEFAULT_ALLOCATOR_CHUNK_SIZE;

		/// Constructor.
		pub const fn new() -> Self
		{
			Self {
				inner_allocator: SimpleMutex::new(None),
			}
		}

		/// Initializes the allocator by feeding it with
		/// backing memory. This operation can be done once.
		pub fn init(
			&self,
			heap: &'a mut [u8],
			bitmap: &'a mut [u8],
		) -> Result<(), GlobalStaticChunkAllocatorError>
		{
			let mut lock = self.inner_allocator.lock();
			if lock.is_some() {
				log::error!("Allocator already initialized!");
				Err(GlobalStaticChunkAllocatorError::AlreadyInitialized)
			} else {
				let alloc = ChunkAllocator::new(heap, bitmap)
					.map_err(|e| GlobalStaticChunkAllocatorError::Inner(e))?;
				log::debug!("initialized the allocator:");
				log::debug!("  chunks: {}", alloc.chunk_count());
				log::debug!("  heap: {} bytes", alloc.capacity());
				lock.replace(alloc);
				Ok(())
			}
		}
	}

	unsafe impl<'a> GlobalAlloc for GlobalStaticChunkAllocator<'a>
	{
		#[track_caller]
		unsafe fn alloc(&self, layout: Layout) -> *mut u8
		{
			// DON'T USE RECURSIVE ALLOCATING HERE
			// LIKE format!().. otherwise infinite loop because of the (dead)lock

			let mut lock = self.inner_allocator.lock();
			let lock = lock.as_mut().expect("allocator is uninitialized");
			let x = lock.alloc(layout);
			// log::debug!(
			// "allocated {} bytes at address 0x{:x}",
			// layout.size(),
			// x as usize
			// );
			x
		}

		#[track_caller]
		unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
		{
			// DON'T USE RECURSIVE ALLOCATING HERE
			// LIKE format!().. otherwise infinite loop because of the (dead)lock

			// log::debug!("dealloc: ptr={:?}, layout={:?}", ptr, layout);
			let mut lock = self.inner_allocator.lock();
			let lock = lock.as_mut().expect("allocator is uninitialized");
			lock.dealloc(ptr, layout)
		}
	}
}
