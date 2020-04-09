// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntry
{
	/// Memory range.
	memory: Range<VirtualAddress>,

	/// Protection.
	pub protection: Protection,

	/// Sharing (can not differentiate persistent shares, sadly).
	pub sharing: Sharing,

	/// Kind of mapping.
	pub kind: MemoryMapEntryKind,

	/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
	pub numa_details: Option<MemoryMapEntryNumaDetails>
}

impl MemoryMapEntry
{
	/// Details for this process.
	#[inline(always)]
	pub fn for_self(proc_path: &ProcPath) -> Result<Vec<Self>, MemoryMapParseError>
	{
		Self::for_process(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Details for a particular process.
	#[inline(always)]
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Vec<Self>, MemoryMapParseError>
	{
		let mut maps_lines = Self::open_file(proc_path, process_identifier, "maps")?.unwrap();
//		let numa_maps_lines = Self::open_file("numa_maps")?;

		let mut parse_state = ParseState::default();
		let mut lines = Vec::new();
		for maps_line in maps_lines.next()
		{
			lines.push(Self::parse_maps_line(&mut parse_state, maps_line?)?);
		}
		parse_state.validate()?;
		Ok(lines)

//		if let Some(numa_maps_lines) = numa_maps_lines
//		{
//		}
	}

	/*
		Example:-
		From-To Perms Offset Maj:Min Inode FilePath/Psuedo/Blank
		55c0f5fd4000-55c0f5fe0000 r--p 00000000 08:03 1048666                    /bin/busybox
		55c0f5fe0000-55c0f607c000 r-xp 0000c000 08:03 1048666                    /bin/busybox
		55c0f607c000-55c0f609d000 r--p 000a8000 08:03 1048666                    /bin/busybox
		55c0f609e000-55c0f60a2000 r--p 000c9000 08:03 1048666                    /bin/busybox
		55c0f60a2000-55c0f60a3000 rw-p 000cd000 08:03 1048666                    /bin/busybox
		55c0f6864000-55c0f6887000 rw-p 00000000 00:00 0                          [heap]
		7f0951b20000-7f0951b35000 r--p 00000000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951b35000-7f0951b7c000 r-xp 00015000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951b7c000-7f0951bb0000 r--p 0005c000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb1000-7f0951bb2000 r--p 00090000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb2000-7f0951bb3000 rw-p 00091000 08:03 2097277                    /lib/ld-musl-x86_64.so.1
		7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0
		7ffc4c759000-7ffc4c77a000 rw-p 00000000 00:00 0                          [stack]
		7ffc4c796000-7ffc4c799000 r--p 00000000 00:00 0                          [vvar]
		7ffc4c799000-7ffc4c79a000 r-xp 00000000 00:00 0                          [vdso]
		Note that the anonymous line (`7f0951bb3000-7f0951bb6000 rw-p 00000000 00:00 0 `) has one space (b' ') after the final, right-hand `0`; this may have been striped by the IDE.
	*/
	#[inline(always)]
	fn parse_maps_line(parse_state: &mut ParseState, (zero_based_line_number, maps_line): (usize, Vec<u8>)) -> Result<Self, MemoryMapParseError>
	{
		parse_state.new_line(zero_based_line_number);
		let mut fields = ParseState::maps_line_split_fields(&maps_line[..]);

		let memory =
		{
			let from = parse_state.next_number_field(&mut fields, "from", VirtualAddress::parse_hexadecimal_number_lower_case)?;
			let to = parse_state.next_number_field(&mut fields, "to", VirtualAddress::parse_hexadecimal_number_lower_case)?;
			parse_state.validate_from_and_to(from, to)?
		};
		let (protection, sharing) = parse_state.next_field(&mut fields, "permissions", |field_bytes, zero_based_line_number, zero_based_field_index| Self::parse_protection_and_sharing(field_bytes, zero_based_line_number, zero_based_field_index))?;
		let offset = parse_state.next_number_field(&mut fields, "offset", Self::parse_offset)?;
		let block_device = BlockDevice
		{
			major: parse_state.next_number_field(&mut fields, "block_device_major", Self::parse_major_or_minor)?,
			minor: parse_state.next_number_field(&mut fields, "block_device_minor", Self::parse_major_or_minor)?,
		};
		let inode = parse_state.next_number_field(&mut fields, "inode", Inode::parse_decimal_number)?;
		let kind =
		{
			let field_bytes = parse_state.last_field(fields, "file_name", |field_bytes, _zero_based_line_number, _zero_based_field_index| Ok(field_bytes))?;
			parse_state.parse_kind(field_bytes, offset, block_device, inode, protection, sharing)?
		};

		Ok
		(
			Self
			{
				memory,
				protection,
				sharing,
				kind,
				numa_details: None,
			}
		)
	}

	#[inline(always)]
	fn open_file(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, file_name: &str) -> Result<Option<impl Iterator<Item=Result<(usize, Vec<u8>), MemoryMapParseError>>>, MemoryMapParseError>
	{
		let file_path = proc_path.process_file_path(process_identifier, file_name);
		if file_path.exists()
		{
			let file = File::open(file_path).map_err(|error| CouldNotOpenFile(error))?;
			let split = BufReader::new(file).split(b'\n');
			let enumerate = split.enumerate();
			let map = enumerate.map(|(zero_based_line_number, result)| result.map(|line| (zero_based_line_number, line)).map_err(|cause| CouldNotReadLine { zero_based_line_number, cause }));
			Ok(Some(map))
		}
		else
		{
			Ok(None)
		}
	}

	#[inline(always)]
	fn parse_protection_and_sharing(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<(Protection, Sharing), MemoryMapParseError>
	{
		if likely!(field_bytes.len() == 4)
		{
			Ok
			(
				(
					Self::parse_protection(field_bytes, zero_based_line_number, zero_based_field_index)?,
					Self::parse_sharing(field_bytes, zero_based_line_number, zero_based_field_index)?
				)
			)
		}
		else
		{
			Err(PermissionsFieldIsWrongLength { zero_based_line_number, zero_based_field_index })
		}
	}

	#[inline(always)]
	fn parse_protection(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<Protection, MemoryMapParseError>
	{
		use self::Protection::*;

		let three_bytes: &[u8; 3] = &field_bytes[0 .. 3].try_into().unwrap();

		let protection = match three_bytes
		{
			b"---" => Unaccessible,
			b"r--" => Read,
			b"rw-" => ReadWrite,
			b"r-x" => ReadExecutable,
			b"rwx" => ReadWriteExecutable,
			_ => return Err(PermissionsFieldUnrecognised { zero_based_line_number, zero_based_field_index, unrecognised: three_bytes.clone() }),
		};
		Ok(protection)
	}

	#[inline(always)]
	fn parse_sharing(field_bytes: &[u8], zero_based_line_number: usize, zero_based_field_index: usize) -> Result<Sharing, MemoryMapParseError>
	{
		use self::Sharing::*;

		let sharing = match unsafe { *field_bytes.get_unchecked(3) }
		{
			b'p' => Private,
			b's' => Shared,
			unrecognised @ _ => return Err(SharingFieldUnrecognised { zero_based_line_number, zero_based_field_index, unrecognised }),
		};
		Ok(sharing)
	}

	#[inline(always)]
	fn parse_offset(field_bytes: &[u8]) -> Result<u32, ParseNumberError>
	{
		u32::parse_hexadecimal_number_lower_case_fixed_width(field_bytes, size_of::<u32>() * 2)
	}

	#[inline(always)]
	fn parse_major_or_minor(field_bytes: &[u8]) -> Result<u8, ParseNumberError>
	{
		u8::parse_hexadecimal_number_lower_case_fixed_width(field_bytes, size_of::<u8>() * 2)
	}
}

#[derive(Default, Debug)]
struct ParseState
{
	zero_based_line_number: usize,
	zero_based_field_index: usize,
	heap: bool,
	stack: bool,
	vdso: bool,
	vvar: bool,
	minimum_next_from: VirtualAddress,
}

impl ParseState
{
	#[inline(always)]
	fn new_line(&mut self, zero_based_line_number: usize)
	{
		self.zero_based_line_number = zero_based_line_number;
		self.zero_based_field_index = 0;
	}

	#[inline(always)]
	fn maps_line_split_fields(maps_line: &[u8]) -> impl Iterator<Item=&[u8]>
	{
		let mut index = 0;
		maps_line.splitn(8, move |byte|
		{
			let separator = match index
			{
				0 => b'-',
				4 => b':',
				_ => b' ',
			};
			if *byte == separator
			{
				index += 1;
				true
			}
			else
			{
				false
			}
		})
	}

	#[inline(always)]
	fn next_number_field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8]) -> Result<P, ParseNumberError>) -> Result<P, MemoryMapParseError>
	{
		self.next_field(fields, name, |field_bytes, zero_based_line_number, zero_based_field_index| parser(field_bytes).map_err(|cause| CouldNotParseNumberField { zero_based_line_number, zero_based_field_index, name, cause }))
	}

	#[inline(always)]
	fn next_field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let result = self.field(fields, name, parser);
		self.zero_based_field_index += 1;
		result
	}

	#[inline(always)]
	fn last_field<'a, P: 'a>(&mut self, mut fields: impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let result = self.field(&mut fields, name, parser);
		debug_assert!(fields.next().is_none());
		result
	}

	#[inline(always)]
	fn field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let field_bytes = fields.next().ok_or(self.missing_required_field(name))?;
		parser(field_bytes, self.zero_based_line_number, self.zero_based_field_index)
	}

	#[inline(always)]
	fn validate_from_and_to(&mut self, from: VirtualAddress, to: VirtualAddress) -> Result<Range<VirtualAddress>, MemoryMapParseError>
	{
		if unlikely!(to <= from)
		{
			return Err(FromAndToAreInvalid { zero_based_line_number: self.zero_based_line_number, from, to })
		}

		if unlikely!(self.minimum_next_from > from)
		{
			return Err(FromTooSmall { zero_based_line_number: self.zero_based_line_number, from, to })
		}

		self.minimum_next_from = to;

		Ok(from .. to)
	}

	#[inline(always)]
	fn parse_kind(&mut self, field_bytes: &[u8], offset: u32, block_device: BlockDevice, inode: Inode, protection: Protection, sharing: Sharing) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		const SpecialFileNameFirstByte: u8 = b'[';
		const FilePathFirstByte: u8 = b'/';

		if field_bytes.is_empty()
		{
			return self.validate_anonymous(offset, block_device, inode)
		}

		let (name, name_first_byte) =
		{
			let name_starts_at = memchr2(SpecialFileNameFirstByte, FilePathFirstByte, field_bytes).ok_or(self.missing_required_field("file_name"))?;
			(&field_bytes[name_starts_at.. ], unsafe { *field_bytes.get_unchecked(name_starts_at) })
		};

		match name_first_byte
		{
			SpecialFileNameFirstByte => self.validate_special_file_name(name, offset, block_device, inode, protection, sharing),

			FilePathFirstByte => self.validate_file(name, offset, block_device, inode),

			_ => unreachable!(),
		}
	}

	#[inline(always)]
	fn validate_anonymous(&self, offset: u32, block_device: BlockDevice, inode: Inode) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		if unlikely!(offset != 0)
		{
			return Err(OffsetWasNotZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, offset })
		}
		if unlikely!(block_device.is_not_zero_zero())
		{
			return Err(BlockDeviceWasNotZeroZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, block_device })
		}
		if unlikely!(inode != Inode::Zero)
		{
			return Err(InodeWasNotZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, inode })
		}

		Ok(MemoryMapEntryKind::Anonymous)
	}

	#[inline(always)]
	fn validate_special_file_name(&mut self, special_file_name: &[u8], offset: u32, block_device: BlockDevice, inode: Inode, protection: Protection, sharing: Sharing) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		use self::MemoryMapEntryKindSpecial::*;
		use self::Protection::*;

		let (present, special_file_name, expected_protection) = match special_file_name
		{
			b"[heap]" => (&mut self.heap, Heap, ReadWrite),
			b"[stack]" => (&mut self.stack, Stack, ReadWrite),
			b"[vdso]" => (&mut self.vdso, vDSO, Read),
			b"[vvar]" => (&mut self.vvar, VVAR, ReadExecutable),

			_ => return Err(UnknownSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name: special_file_name.to_vec() }),
		};

		if unlikely!(*present)
		{
			return Err(RepeatedSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name })
		};

		*present = true;

		if unlikely!(offset != 0)
		{
			return Err(OffsetWasNotZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, offset })
		}
		if unlikely!(block_device.is_not_zero_zero())
		{
			return Err(BlockDeviceWasNotZeroZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, block_device })
		}
		if unlikely!(inode != Inode::Zero)
		{
			return Err(InodeWasNotZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, inode })
		}
		if unlikely!(protection != expected_protection)
		{
			return Err(ProtectionWasNotExpectedForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, protection, expected_protection })
		}
		if unlikely!(sharing != Sharing::Private)
		{
			return Err(SharingWasNotPrivateForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, sharing })
		}

		Ok(MemoryMapEntryKind::Special(special_file_name))
	}

	#[inline(always)]
	fn validate_file(&self, new_line_escaped_file_path_which_may_be_deleted: &[u8], offset: u32, block_device: BlockDevice, inode: Inode) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		let (new_line_escaped_file_path, deleted) = without_suffix(new_line_escaped_file_path_which_may_be_deleted, b" (deleted)");
		let unescaped_file_path_bytes = Self::unescape_file_path(new_line_escaped_file_path.to_vec());

		Ok
		(
			MemoryMapEntryKind::File
			{
				offset,
				block_device,
				inode,
				file_path: PathBuf::from(OsString::from_vec(unescaped_file_path_bytes)),
				deleted,
				numa_details: None
			}
		)
	}

	// Works backwards as that requires copy operations to move fewer bytes.
	fn unescape_file_path(mut path_with_escaped_new_lines: Vec<u8>) -> Vec<u8>
	{
		const StartOfEscapeSequence: u8 = b'\\';
		const StartOfEscapeSequenceLength: usize = 1;

		const RemainingEscapeSequence: &'static [u8] = b"012";
		const RemainingEscapeSequenceLength: usize = 3;

		const EscapeSequenceLength: usize = StartOfEscapeSequenceLength + RemainingEscapeSequenceLength;

		let mut unescaped_length = path_with_escaped_new_lines.len();
		{
			let mut remaining_bytes = &mut path_with_escaped_new_lines[..];
			while likely!(remaining_bytes.len() >= EscapeSequenceLength)
			{
				let index = match memrchr(StartOfEscapeSequence, &remaining_bytes[ .. remaining_bytes.len() - RemainingEscapeSequenceLength])
				{
					None => break,
					Some(index) => index,
				};

				let inclusive_start_index = index + 1;
				let exclusive_end_index = inclusive_start_index + RemainingEscapeSequenceLength;
				if &remaining_bytes[inclusive_start_index .. exclusive_end_index] == b"012"
				{
					unsafe
					{
						*remaining_bytes.get_unchecked_mut(index) = b'\n';
						let from = remaining_bytes.as_ptr().offset(exclusive_end_index as isize);
						let to = remaining_bytes.as_mut_ptr().offset(inclusive_start_index as isize);
						from.copy_to(to, remaining_bytes.len() - exclusive_end_index);
						unescaped_length -= RemainingEscapeSequenceLength;
					}
				}
				remaining_bytes = &mut remaining_bytes[ .. index]
			}
		}
		unsafe { path_with_escaped_new_lines.set_len(unescaped_length) };
		path_with_escaped_new_lines.shrink_to_fit();
		path_with_escaped_new_lines
	}

	#[inline(always)]
	const fn missing_required_field(&self, name: &'static str) -> MemoryMapParseError
	{
		MissingRequiredField { zero_based_line_number: self.zero_based_line_number, zero_based_field_index: self.zero_based_field_index, name }
	}

	#[inline(always)]
	fn validate(&self) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(!self.stack)
		{
			return Err(MissingStackMapping)
		}

		if unlikely!(!self.vdso)
		{
			return Err(MissingVdsoMapping)
		}

		if unlikely!(!self.vvar)
		{
			return Err(MissingVvarMapping)
		}

		Ok(())
	}
}
