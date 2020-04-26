// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represent a `nice` value.
///
/// For setting the current thread niceness, use the more modern `Scheduler`.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum Nice
{
	/// Least priority.
	Positive_19 = 19,

	Positive_18 = 18,

	Positive_17 = 17,

	Positive_16 = 16,

	Positive_15 = 15,

	Positive_14 = 14,

	Positive_13 = 13,

	Positive_12 = 12,

	Positive_11 = 11,

	Positive_10 = 10,

	Positive_9 = 9,

	Positive_8 = 8,

	Positive_7 = 7,

	Positive_6 = 6,

	Positive_5 = 5,

	Positive_4 = 4,

	Positive_3 = 3,

	Positive_2 = 2,

	Positive_1 = 1,

	Zero = 0,

	Negative_1 = -1,

	Negative_2 = -2,

	Negative_3 = -3,

	Negative_4 = -4,

	Negative_5 = -5,

	Negative_6 = -6,

	Negative_7 = -7,

	Negative_8 = -8,

	Negative_9 = -9,

	Negative_10 = -10,

	Negative_11 = -11,

	Negative_12 = -12,

	Negative_13 = -13,

	Negative_14 = -14,

	Negative_15 = -15,

	Negative_16 = -16,

	Negative_17 = -17,

	Negative_18 = -18,

	Negative_19 = -19,

	/// Highest priority.
	Negative_20 = -20,
}

impl Display for Nice
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", *self as i32)
	}
}

impl Default for Nice
{
	#[inline(always)]
	fn default() -> Self
	{
		Nice::Negative_20
	}
}

impl Into<i8> for Nice
{
	#[inline(always)]
	fn into(self) -> i8
	{
		self as i32 as i8
	}
}

impl Into<i16> for Nice
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self as i32 as i16
	}
}

impl Into<i32> for Nice
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Into<i64> for Nice
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self as i32 as i64
	}
}

impl Into<isize> for Nice
{
	#[inline(always)]
	fn into(self) -> isize
	{
		self as i32 as isize
	}
}

impl ParseNumber for Nice
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		use self::ParseNumberError::*;

		let value = i32::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(value < Self::InclusiveMinimum)
		{
			Err(TooSmall)
		}
		else if unlikely!(value > Self::InclusiveMaximum)
		{
			Err(TooLarge)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl Nice
{
	pub(super) const InclusiveMinimum: i32 = -20;

	pub(super) const InclusiveMaximum: i32 = 19;

	/// Set the autogroup for the current process.
	#[inline(always)]
	pub fn set_autogroup_for_current_process(self, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		self.set_autogroup_for_process(ProcessIdentifierChoice::Current, proc_path)
	}

	/// Set the autogroup for a process.
	#[inline(always)]
	pub fn set_autogroup_for_process(self, process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		proc_path.process_file_path(process_identifier, "autogroup").write_value(self as i32)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn set_current_process_priority(self) -> Result<(), ()>
	{
		self.set_process_priority(ProcessIdentifierChoice::Current)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_process_priority(self, process_identifier: ProcessIdentifierChoice) -> Result<(), ()>
	{
		let pid: pid_t = process_identifier.into();
		self.set_priority(PRIO_PROCESS, pid as u32)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn set_current_process_group_priority(self) -> Result<(), ()>
	{
		self.set_process_group_priority(ProcessGroupIdentifierChoice::Current)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_process_group_priority(self, process_group_identifier: ProcessGroupIdentifierChoice) -> Result<(), ()>
	{
		let pgid: pid_t = process_group_identifier.into();
		self.set_priority(PRIO_PGRP, pgid as u32)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn set_current_user_priority(self) -> Result<(), ()>
	{
		const CurrentNotRoot: u32 = 0;
		self.set_priority(PRIO_USER, CurrentNotRoot)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_user_priority(self, user_identifier: UserIdentifier) -> Result<(), ()>
	{
		let uid: uid_t = user_identifier.into();
		self.set_priority(PRIO_USER, uid)
	}

	#[inline(always)]
	fn set_priority(self, which: i32, who: u32) -> Result<(), ()>
	{
		let result = unsafe { setpriority(which, who, self as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("`which` was not one of `PRIO_PROCESS`, `PRIO_PGRP`, or `PRIO_USER`"),
				ESRCH => panic!("no process was located using the `which` and `who` values specified"),
				EACCES | EPERM => Err(()),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}