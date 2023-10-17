// SPDX-License-Identifier: GPL-3.0-or-later

/// ### The Kernel Target
///
/// The kernel target is a triple consisting of
///
/// 1. The hardware architecture                        aarch64
/// 2. The vendor (manufacturer) (optional)             unknown
/// 3. Operating system                                 uncore
/// 4. ABI (optional, omitted in our case)
///
/// The target triple reads as `ARCH-VENDOR-SYS-ABI` and you can read
/// about it [here](https://docs.rust-embedded.org/embedonomicon/custom-target.html).
///
/// The default case for `unCORE` is `x86_64-unknown-uncore`. This
/// target is for freestanding / bare-metal `x86-64` binaries in ELF
/// format, i.e. firmware, kernels, etc.
const BUILD_TARGET: Option<&str> = option_env!("BUILD_TARGET");

/// ### Compilation Date and Time
///
/// Contains the output of `date +'%H:%M, %d %b %Y'` right before the
/// kernel was compiled.
const COMPILATION_DATE_AND_TIME: Option<&str> = option_env!("COMPILATION_DATE_AND_TIME");

/// ### Kernel Version
///
/// The `KERNEL_VERSION` variable contains the kernel version in the
/// semantic versioning format, the git commit id the kernel was built
/// with and the build date. If `KERNEL_VERSION` was not available
/// during build-time, a default value is provided, namely "testing".
const KERNEL_VERSION: Option<&str> = option_env!("KERNEL_VERSION");

/// ### Rust Toolchain
///
/// Holds the toolchain information that this version of the kernel
/// (stored in `KERNEL_VERSION`) was compiled with.
const RUST_TOOLCHAIN: Option<&str> = option_env!("RUST_TOOLCHAIN");

/// ### Compiler Version
///
/// This variable holds the compiler version that this version of the
/// kernel (stored in `KERNEL_VERSION`) was compiled with.
const RUSTC_VERSION: Option<&str> = option_env!("RUSTC_VERSION");

/// ### Static Kernel Information
///
/// This struct exists to call non-member ("static") function on it to
/// obtain information about the kernel, such as its version or build
/// target as a string.
#[derive(Debug, Copy, Clone)]
pub struct KernelInformation;

impl KernelInformation
{
	/// ### Kernel Build Target
	///
	/// Returns the (LLVM) target triple if provided at
	/// built-time. This function tries to make a best-guess by
	/// using conditional compilation if the environment variable
	/// `BUILD_TARGET` was not specified. If nothing was found,
	/// "unknown" is returned.
	#[must_use]
	pub fn get_build_target() -> &'static str
	{
		#[cfg(target_arch = "aarch64")]
		let target_triple = "aarch64-unknown-uncore";

		#[cfg(target_arch = "i686")]
		let target_triple = "i686-unknown-uncore";

		#[cfg(target_arch = "x86_64")]
		let target_triple = "x86_64-unknown-uncore";

		BUILD_TARGET.unwrap_or(target_triple)
	}

	/// ### Kernel Compilation Date and Time
	///
	/// Returns the kernel's build date and time, if the
	/// corresponding environment variable was present, otherwise
	/// returns "unknown".
	#[must_use]
	pub fn get_compilation_date_and_time() -> &'static str
	{
		COMPILATION_DATE_AND_TIME.unwrap_or("unknown")
	}

	/// ### Kernel Version
	///
	/// Returns the kernel version if provided at built-time,
	/// otherwise returns "testing".
	#[must_use]
	pub fn get_kernel_version() -> &'static str { KERNEL_VERSION.unwrap_or("testing") }

	/// ### Kernel Rust Toolchain Information
	///
	/// Returns the toolchain information that this version of the
	/// kernel was compiled with if the environment variable was
	/// provided at built-time, otherwise returns "unknown".
	#[must_use]
	pub fn get_rust_toolchain() -> &'static str { RUST_TOOLCHAIN.unwrap_or("unknown") }

	/// ### Kernel Compiler Version
	///
	/// Returns the version of `rustc` that this version of the
	/// kernel was compiled with if the environment variable was
	/// provided at built-time, otherwise returns "unknown".
	#[must_use]
	pub fn get_rustc_version() -> &'static str { RUSTC_VERSION.unwrap_or("unknown") }
}

/// ## Boot-Information Abstraction
///
/// Provides a type that abstracts over boot information provided by bootloaders using
/// conditional compilation.
pub mod boot
{
	/// ### Opaque Boot Information
	///
	/// This structure exists to have a uniform type for different architectures to
	/// work with their boot information (provided by the bootloader). This way, we
	/// need not write a different `fn main()`, etc. for every architecture.
	#[cfg(target_arch = "x86_64")]
	#[derive(Debug)]
	pub struct Information(pub &'static bootloader::BootInfo);
}

/// ## Kernel Library Types
///
/// This module contains important data types (enums, structures,
/// etc.) used throughout the kernel. One example includes the
/// [`kernel_types::GlobalStaticMut`] type used for global static variables.
pub mod kernel_types
{
	/// ### Kernel Exit Code
	///
	/// Shows whether the kernel exited successfully or not.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum ExitCode
	{
		/// Exit with success
		Success = 0,
		/// Exit with failure
		Failure = 1,
	}

	/// ### Global Static Variables for Non-Thread-Safe Types
	///
	/// This enumeration is the abstraction needed to abstract
	/// over global `static` variables. This is case because these
	/// variables need to be thread safe, and some types do not
	/// implement [`Send`] or [`Sync`]. Furthermore, when the
	/// kernel boots, there is no allocator (needed by
	/// [`alloc::sync::Arc`]). Therefore, this type eliminates
	/// the hassle of working with [`alloc::sync::Arc`] or
	/// [`spin::Mutex`].
	///
	/// #### Safety
	///
	/// Calling the [`Self::new`] is always safe, but calling
	/// [`Self::initialize`] requires a global allocator to be set
	/// up.
	#[allow(clippy::non_send_fields_in_send_ty)]
	#[derive(Debug)]
	pub enum GlobalStaticMut<T>
	{
		/// The default, boot-time state
		Uninitialized,
		/// The "runtime", post-boot state
		Initialized(alloc::sync::Arc<lock::Locked<T>>),
	}

	impl<T> GlobalStaticMut<T>
	{
		/// ### Constant Boot-Time Initialization
		///
		/// This function **must** be used when creating a global, static variable
		/// for non-thread safe types.
		#[must_use]
		pub const fn new() -> Self { Self::Uninitialized }

		/// ### Initializing - Post-Boot
		///
		/// This function will return the [`Self::Initialized`] state with
		/// initialized data. It will wrap the type in an
		/// [`alloc::sync::Arc<spin::Mutex<T>>`] for thread-safe operation.
		///
		/// #### Safety
		///
		/// This function is considered unsafe for two reasons:
		///
		/// 1. It operates and changes `static mut` variables, which itself is
		/// `unsafe`
		/// 2. When calling this function before a global allocator has
		///    been setup, this function    will panic due to the need for an
		///    allocator in [`alloc::sync::Arc`].
		pub unsafe fn initialize(inner_value: T) -> Self
		{
			Self::Initialized(alloc::sync::Arc::new(lock::Locked::from(inner_value)))
		}

		/// ### Check Status
		///
		/// Checks whether the variable is initialized or not. Returns true if the
		/// variable is initialized.
		#[must_use]
		pub const fn is_initialized(&self) -> bool
		{
			match self {
				Self::Uninitialized => false,
				Self::Initialized(_) => true,
			}
		}

		/// ### Get Exclusive Access
		///
		/// Returns a guard to the inner data field, that provides
		/// (mutable) access to the encapsulated data, if the data
		/// has already been initialized. If this is not the case,
		/// this function returns [`None`].
		///
		/// #### Safety
		///
		/// This function is marked as `unsafe` because access to
		/// the underlying data appears to be [`Send`] and [`Sync`] while these
		/// traits are actually implemented for all (generic) types this structure
		/// encapsulates, even if these types are not strictly [`Send`] or
		/// [`Sync`]. That being said, the access **should be** safe, but you'll
		/// need to take care nevertheless.
		#[must_use]
		pub unsafe fn lock(&self) -> Option<spin::MutexGuard<T>>
		{
			if let Self::Initialized(data) = self {
				Some(data.lock())
			} else {
				None
			}
		}
	}

	unsafe impl<T> Send for GlobalStaticMut<T> {}
	unsafe impl<T> Sync for GlobalStaticMut<T> {}

	/// ## Kernel Error Types
	///
	/// Contains various error abstractions for different parts of the kernel,
	/// including errors when handling virtual memory, etc.
	///
	/// The **idea of this module** is to provide types that make using the `?`
	/// operator easy. Using the `?` operator makes life easy and code nice. Moreover,
	/// one does not need to bother with checking every error at all level, but let
	/// the caller decide what to do: propagate the error again, or handle it.
	pub mod errors
	{
		use ::core::fmt;

		/// ### Kernel Errors
		///
		/// All error structures / enumerations of the kernel must implement this
		/// trait. It requires [`::core::fmt::Debug`] and [`::core::fmt::Display`]
		/// to be implemented on the type.
		pub trait Error: fmt::Debug + fmt::Display
		{
			/// ### Whether Another Error is the Cause
			///
			/// If the error itself is caused by another error, this method
			/// can be used to obtain it. Returns [`None`] is there is no
			/// other error source.
			fn source(&self) -> Option<&(dyn Error + 'static)> { None }

			/// ### Error Backtrace
			///
			/// Provides a "backtrace" of the error to see where is
			/// originated.
			///
			/// #### The Default Implementation
			///
			/// The default implementation will just [`panic!`] by calling the
			/// [`todo!`] macro.
			fn backtrace(&self)
			{
				todo!("error backtracing");
			}
		}

		/// ### Errors for Virtual Memory
		///
		/// Contains variants needed when dealing with virtual memory.
		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum VirtualMemory
		{
			/// For address alignment issues.
			AddressNotAligned,
			/// A frame could not be allocated.
			FrameAllocationFailed,
			/// A page could not be mapped.
			PageMappingError,
			/// A page could not be allocated.
			PageAllocationFailed,
			/// A soft-error during page deallocation that indicates that the
			/// address was not actually mapped.
			PageDeallocationPageWasNotMapped,
		}

		impl fmt::Display for VirtualMemory
		{
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
			{
				let variant = match &self {
					Self::AddressNotAligned => "address is not aligned",
					Self::FrameAllocationFailed => "frame allocation failed",
					Self::PageMappingError => "page could not be mapped",
					Self::PageAllocationFailed => "page allocation failed",
					Self::PageDeallocationPageWasNotMapped => {
						"no deallocation as page was not mapped"
					},
				};

				write!(f, "virtual memory error (issue: {})", variant)
			}
		}

		impl Error for VirtualMemory {}
	}

	/// ## Kernel Wide Locking Abstraction
	///
	/// This module abstracts over a specific locking mechanism to provide unified
	/// locking in the whole kernel.
	///
	/// Currently, a simple [`spin::Mutex`] is used to lock and achieve [`Sync`].
	pub mod lock
	{
		/// ### The Locking Structure
		///
		/// This structure abstracts over its inner data and provides [`Sync`]
		/// access to it if the provided data is [`Send`].
		#[derive(Debug)]
		pub struct Locked<T>
		{
			/// Only and inner data field
			data: spin::Mutex<T>,
		}

		impl<T> Locked<T>
		{
			/// ### Create a New Locked Structure
			///
			/// Encapsulates the given `data`, taking ownership over it, and
			/// locking it.
			pub const fn from(data: T) -> Self
			{
				Self {
					data: spin::Mutex::new(data),
				}
			}

			/// ### Get Exclusive Access
			///
			/// Returns a guard to the inner data field, that provides
			/// (mutable) access to the encapsulated data.
			pub fn lock(&self) -> spin::MutexGuard<T> { self.data.lock() }
		}
	}
}

/// ## QEMU Abstractions
///
/// Contains helpers for running the kernel with QEMU.
pub mod qemu
{
	/// ### Write An Exit Code
	///
	/// Writes to the `0xF4` port the correct bytes that indicate
	/// either success or failure.
	#[inline]
	fn exit(success: bool)
	{
		use qemu_exit::QEMUExit;

		if runs_inside_qemu::runs_inside_qemu().is_definitely_not() {
			return;
		}

		#[cfg(target_arch = "x86_64")]
		let qemu_exit_handle = qemu_exit::X86::new(0xF4, 0x3);

		if success {
			qemu_exit_handle.exit_success();
		} else {
			qemu_exit_handle.exit_failure();
		}
	}

	/// ### Exit QEMU With Success
	///
	/// Write a success exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_success() { exit(true); }

	/// ### Exit QEMU Without Success
	///
	/// Write a failure exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_failure() { exit(false); }
}
