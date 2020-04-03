// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Commonly supported huge page sizes for modern popular CPU architectures (x86, ARM, PowerPC).
///
/// See also <https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages>.
///
/// `repr(u64)` values are in KiloBytes.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HugePageSize
{
	/// 64Kb.
	_64KB = 64,

	/// 512Kb.
	_512KB = 512,

	/// 1MB.
	_1MB = 1024,
	
	/// 2MB.
	_2MB = 2048,
	
	/// 4MB.
	_4MB = 4096,

	/// 8MB.
	_8MB = 8192,
	
	/// 16MB.
	_16MB = 16_384,

	/// 32MB.
	_32MB = 32_768,
	
	/// 256MB.
	_256MB = 262_144,
	
	/// 512MB.
	///
	/// aarch64 alternative.
	_512MB = 524_288,

	/// 1GB.
	_1GB = 1_048_576,
	
	/// 2GB.
	_2GB = 2_097_152,
	
	/// 16GB.
	_16GB = 16_777_216,
}

impl HugePageSize
{
	/// Potentially supported huge page sizes.
	///
	/// Range based on defined constants `MAP_HUGE_*`.
	pub const PotentiallySupportedHugePageSizesLargestFirst: [HugePageSize; 13] =
	[
		HugePageSize::_16GB,
		HugePageSize::_2GB,
		HugePageSize::_1GB,
		HugePageSize::_512MB,
		HugePageSize::_256MB,
		HugePageSize::_32MB,
		HugePageSize::_16MB,
		HugePageSize::_8MB,
		HugePageSize::_4MB,
		HugePageSize::_2MB,
		HugePageSize::_1MB,
		HugePageSize::_512KB,
		HugePageSize::_64KB,
	];

	/// What value, N, gives `Self == 1 << N`.
	const fn log_base_2_of_bytes(self) -> u64
	{
		(self as u64).trailing_zeros() as u64
	}

	/// Value for use with `flags` in `mmap()` (`MAP_HUGE_*`, eg `MAP_HUGE_2MB`) and `memfd_create()` (`MFD_HUGE_*` eg MFD_HUGE_2MB).
	///
	/// Note that the `mmap()` and `memfd_create()` calls can not accept huge page sizes over 2Gb as they use an unsigned integer!
	///
	/// The 6 bits `[26:31]` of the flag arguments encode the log2 of the huge page size in kilobytes.
	#[inline(always)]
	pub const fn mmap_and_memfd_flags_bits(self) -> u64
	{
		/*
			                    self.log_base_2()
			                    ^
			 1*        =>       0
			 2*        =>       1
			 4*        =>       2
			 8*        =>       3
			16*        =>       4
			32*        =>       5
			64         => 16 -  6 = 10
			128*       =>       7
			256*       =>       8
			512        => 19 -  9 = 10
			1024       => 20 - 10 = 10
			16_777_216 => 34 - 24 = 10
		*/
		const Log2OfBytesInAKilobyte: u64 = 10;
		const MAP_HUGE_SHIFT: u64 = 26;
		(self.log_base_2_of_bytes() - Log2OfBytesInAKilobyte) << MAP_HUGE_SHIFT
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn can_have_a_dynamic_huge_page_pool(self) -> bool
	{
		!self.is_a_gigantic_huge_page()
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn is_a_gigantic_huge_page(self) -> bool
	{
		const Scalar: u64 = 2048;

		let minimum_gigantic_huge_page = (page_size() as u64) * Scalar;
		self.size_in_bytes() >= minimum_gigantic_huge_page
	}
	
	/// Size in mega bytes.
	#[inline(always)]
	pub fn size_in_mega_bytes(self) -> u64
	{
		self.size_in_kilo_bytes() / 1024
	}
	
	/// Size in kilo bytes.
	#[inline(always)]
	pub fn size_in_kilo_bytes(self) -> u64
	{
		self as u64
	}
	
	/// Size in bytes.
	#[inline(always)]
	pub fn size_in_bytes(self) -> u64
	{
		self.size_in_kilo_bytes() * 1024
	}
	
	/// Calculate number of huge pages.
	#[inline(always)]
	pub fn calculate_number_of_huge_pages(&self, desired_number_of_kilo_bytes: u64) -> u64
	{
		let size_in_kilo_bytes = self.size_in_kilo_bytes();
		if size_in_kilo_bytes < desired_number_of_kilo_bytes
		{
			1
		}
		else
		{
			size_in_kilo_bytes / desired_number_of_kilo_bytes
		}
	}
	
	/// Converts a value from Linux's `/proc/mem` pseudo-file into a `HugePageSize`.
	#[inline(always)]
	pub fn from_proc_mem_info_value(value: u64) -> Option<Self>
	{
		use self::HugePageSize::*;
		
		match value
		{
			64 => Some(_64KB),
			512 => Some(_512KB),
			1_024 => Some(_1MB),
			2_048 => Some(_2MB),
			4_096 => Some(_4MB),
			8_192 => Some(_8MB),
			16_384 => Some(_16MB),
			32_768 => Some(_32MB),
			262_144 => Some(_256MB),
			524_288 => Some(_512MB),
			1_048_576 => Some(_1GB),
			2_097_152 => Some(_2GB),
			16_777_216 => Some(_16GB),
			
			_ => None,
		}
	}
	
	/// String description including unit.
	#[inline(always)]
	pub fn to_str(&self) -> &'static str
	{
		use self::HugePageSize::*;
		
		match *self
		{
			_64KB => "64KB",
			_512KB => "512KB",
			_1MB => "1MB",
			_2MB => "2MB",
			_4MB => "4MB",
			_8MB => "8MB",
			_16MB => "16MB",
			_32MB => "32MB",
			_256MB => "256MB",
			_512MB => "512MB",
			_1GB => "1GB",
			_2GB => "2GB",
			_16GB => "16GB",
		}
	}
	
	/// String description including unit.
	#[inline(always)]
	pub fn to_bytes(&self) -> &'static [u8]
	{
		use self::HugePageSize::*;
		
		match *self
		{
			_64KB => b"64KB",
			_512KB => b"512KB",
			_1MB => b"1MB",
			_2MB => b"2MB",
			_4MB => b"4MB",
			_8MB => b"8MB",
			_16MB => b"16MB",
			_32MB => b"32MB",
			_256MB => b"256MB",
			_512MB => b"512MB",
			_1GB => b"1GB",
			_2GB => b"2GB",
			_16GB => b"16GB",
		}
	}

	/// Default huge page size.
	///
	/// Usually 2Mb on x86-64 (but controlled by kernel command line options).
	#[inline(always)]
	pub fn default_huge_page_size(memory_information: &MemoryInformation) -> Option<Self>
	{
		if let Some(size_in_bytes) = memory_information.get_statistic(&MemoryInformationName::SizeOfDefaultHugePage)
		{
			Self::from_proc_mem_info_value(size_in_bytes)
		}
		else
		{
			None
		}
	}

	/// May be absent on some Kernels.
	#[inline(always)]
	pub fn transparent_huge_page_size(sys_path: &SysPath) -> Option<Self>
	{
		let file_path = sys_path.transparent_huge_memory_file_path("hpage_pmd_size");
		if file_path.exists()
		{
			let value: u64 = file_path.read_value().unwrap();
			Self::from_proc_mem_info_value(value)
		}
		else
		{
			None
		}
	}

	/// Largest supported huge page size.
	#[inline(always)]
	pub fn largest_supported_huge_page_size(sys_path: &SysPath) -> Self
	{
		*Self::supported_huge_page_sizes(sys_path).iter().rev().next().expect("Huge pages are not supported")
	}
	
	/// Supported huge page sizes, sorted smallest to largest.
	///
	/// There may be no huge pages because:-
	///
	/// * The folder `/sys/kernel/mm/hugepages` does not exist (I have seen this on Alpine Linux 3.11 running the 'linux-lts' kernel on a Parallels Hypervisor).
	/// * The architecture only has huge pages we do not support (extremely unlikely).
	/// * We are on an ancient CPU that does not have huge pages (extremely unlikely).
	///
	/// On modern x86-64 from Sandy Bridge onwards, will contain 1Gb and 2Mb huge page sizes.
	#[inline(always)]
	pub fn supported_huge_page_sizes(sys_path: &SysPath) -> BTreeSet<Self>
	{
		let mut supported = BTreeSet::new();
		
		for huge_page_size in Self::PotentiallySupportedHugePageSizesLargestFirst.iter()
		{
			let huge_page_size = *huge_page_size;
			if sys_path.global_hugepages_folder_path(huge_page_size).exists()
			{
				supported.insert(huge_page_size);
			}
		}
		
		supported
	}

	/// Huge page pool statistics.
	///
	/// For per-NUMA Node statistics, see `NumaNode.huge_page_pool_statistics()`.
	///
	/// This will panic if the kernel was compiled without `CONFIG_HUGETLBFS` and the `hugepages` folder is missing under the `node<N>` folder.
	#[inline(always)]
	pub fn global_huge_page_pool_statistics(self, sys_path: &SysPath) -> HugePagePoolStatistics
	{
		HugePagePoolStatistics::new(sys_path, self, SysPath::global_hugepages_folder_path)
	}

	/// Read number of memory policy global huge pages of `self` size.
	///
	/// This file is missing if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn memory_policy_global_huge_pages(self, sys_path: &SysPath) -> Option<u64>
	{
		let file_path = sys_path.global_hugepages_folder_path(self).append("nr_hugepages_mempolicy");

		if file_path.exists()
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			return None
		}
	}
}