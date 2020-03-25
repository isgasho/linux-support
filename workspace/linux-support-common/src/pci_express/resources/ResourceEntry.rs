// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A resource entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceEntry
{
	/// Inclusive start.
	pub inclusive_start: NonNull<u8>,

	/// Inclusive end.
	pub inclusive_end: NonNull<u8>,

	/// Flags.
	pub flags: u64,
}

impl ResourceEntry
{
	/// Length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.inclusive_end.as_ptr() as usize + 1 - self.inclusive_start.as_ptr() as usize
	}

	#[inline(always)]
	pub(crate) fn memory_map<'a>(pci_device: &PciDevice<'a>, resource_index: u8) -> Result<MemoryMappedResource, io::Error>
	{
		let resource_file_path = pci_device.device_file_or_folder_path(&format!("resource{:?}", resource_index));

		let file = OpenOptions::new().read(true).write(true).open(&resource_file_path)?;

		let size =
		{
			let metadata = resource_file_path.metadata()?;
			if !metadata.is_file()
			{
				return Err(io::Error::from(ErrorKind::Other))
			}
			metadata.len() as usize
		};

		let result = unsafe { mmap(null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, file.as_raw_fd(), 0) };
		if unlikely!(result == MAP_FAILED)
		{
			return Err(io::Error::last_os_error())
		}

		Ok
		(
			MemoryMappedResource
			{
				pointer: unsafe { NonNull::new_unchecked(result as *mut u8) },
				size,
			}
		)
	}

	/// A typical line might be `0x0000000000008200 0x000000000000823f 0x0000000000040101`.
	fn parse_line(line: &str) -> Result<Option<Self>, &'static str>
	{
		#[inline(always)]
		fn parse_u64_hexadecimal_value<'a>(iterator: &mut impl Iterator<Item=&'a str>) -> Result<u64, &'static str>
		{
			// eg `0x0000000000008200`.
			const Prefix: &'static str = "0x";
			const HexadecimalDigits: usize = 16;

			let hexadecimal_prefixed_string = iterator.next().ok_or("Missing expected field")?;
			if unlikely!(hexadecimal_prefixed_string.len() != (Prefix.len() + HexadecimalDigits))
			{
				return Err("Invalid length")
			}
			if unlikely!(!hexadecimal_prefixed_string.starts_with("0x"))
			{
				return Err("Does not start with '0x' prefix")
			}
			(&hexadecimal_prefixed_string[Prefix.len() .. ]).parse().map_err(|_| "Invalid hexadecimal string")
		}

		let mut iterator = line.splitn(3, ' ');
		let start = parse_u64_hexadecimal_value(&mut iterator)?;
		let end = parse_u64_hexadecimal_value(&mut iterator)?;
		let flags = parse_u64_hexadecimal_value(&mut iterator)?;

		if start == 0 && end == 0 && flags == 0
		{
			return Ok(None)
		}

		if unlikely!(start == 0)
		{
			return Err("start is zero");
		}

		if unlikely!(start >= end)
		{
			return Err("start is greater than or equal to end");
		}

		Ok
		(
			Some
			(
				Self
				{
					inclusive_start: unsafe { NonNull::new_unchecked(start as *mut u8) },
					inclusive_end: unsafe { NonNull::new_unchecked(end as *mut u8) },
					flags,
				}
			)
		)
	}
}
