// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Regular page size in bytes for the current process.
///
/// Some MIPS processors supports 1Kb and 2Kb page sizes, but Linux does not.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u64)]
pub enum PageSize
{
	/// 4Kb page size.
	///
	/// The only page size possible on powerpc64, riscv64 and x86_64.
	/// mips64 but not Loongson 2EF or Loongson 64.
	/// aarch64 with the 4Kb translation granule.
	_4Kb = 4_096,

	/// 8Kb page size.
	///
	/// The only page size possible on sparc64.
	/// mips64 for Cavium Octeon cnMIPS processors only.
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))]
	_8Kb = 8_192,

	/// 16Kb page size.
	///
	/// mips64 and aarch64 as with 16Kb translation granule.
	#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))]
	_16Kb = 16_384,

	/// 32Kb page size.
	///
	/// mips64 for Cavium Octeon cnMIPS processors only.
	#[cfg(target_arch = "mips64")]
	_32Kb = 32_768,

	/// 64Kb page size.
	///
	/// mips64 and aarch64 with 64Kb translation granule.
	#[cfg(any(target_arch = "aarch64", target_arch = "mips64"))]
	_64Kb = 65_536,
}

impl Default for PageSize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::current()
	}
}

impl PageSize
{
	/// Size in kilobytes.
	#[inline(always)]
	pub const fn size_in_kilobytes(self) -> NonZeroKilobyte
	{
		unsafe { NonZeroU64::new_unchecked((self as u64) / 1_024) }
	}

	/// Size in bytes.
	#[inline(always)]
	pub const fn size_in_bytes(self) -> NonZeroU64
	{
		unsafe { NonZeroU64::new_unchecked(self as u64) }
	}

	/// Non-zero number of pages from non-zero number of bytes, rounded up.
	#[inline(always)]
	pub fn non_zero_number_of_pages_from_non_zero_number_of_bytes_rounded_up(self, number_of_bytes: NonZeroU64) -> NonZeroNumberOfPages
	{
		unsafe { NonZeroU64::new_unchecked(self.number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes.get())) }
	}

	/// Number of pages from number of bytes, rounded up.
	#[inline(always)]
	pub fn number_of_pages_from_number_of_bytes_rounded_up(self, number_of_bytes: u64) -> NumberOfPages
	{
		let size_in_bytes = self.size_in_bytes().get();
		(number_of_bytes + size_in_bytes - 1) / size_in_bytes
	}

	/// Non-zero number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: NonZeroU64) -> NonZeroU64
	{
		unsafe { NonZeroU64::new_unchecked(self.number_of_bytes_rounded_up_to_multiple_of_page_size(number_of_bytes.get())) }
	}

	/// Number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: u64) -> u64
	{
		let size_in_bytes = self.size_in_bytes().get();
		((number_of_bytes + size_in_bytes - 1) / size_in_bytes) * size_in_bytes
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn can_have_a_dynamic_huge_page_pool(self) -> bool
	{
		false
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn is_a_gigantic_huge_page(self) -> bool
	{
		false
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, riscv64, sparc64 and x86_64, the value is trully constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "x86_64"))]
	#[inline(always)]
	pub const fn current() -> Self
	{
		Self::_4Kb
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, sparc64 and x86_64, the value is trully constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(target_arch = "sparc64")]
	#[inline(always)]
	pub const fn current() -> Self
	{
		Self::_8Kb
	}

	/// A very slightly faster function to get page size than `sysconf(_SC_PAGESIZE)` on musl libc systems.
	///
	/// On powerpc64, sparc64 and x86_64, the value is trully constant; otherwise, it is effectively constant and is derived from data passed when an executable is first loaded.
	#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))]
	#[inline(always)]
	pub fn current() -> Self
	{
		// `getpagesize()` is faster than `sysconf(_SC_PAGESIZE)` on musl libc systems.
		unsafe { transmute(getpagesize() as usize) }
	}

}