// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process group identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ProcessGroupIdentifier(NonZeroI32);

impl Default for ProcessGroupIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		let pid = unsafe { getpgid(0) };
		debug_assert!(pid > 0);
		Self(unsafe { NonZeroI32::new_unchecked(pid)})
	}
}

impl TryFrom<pid_t> for ProcessGroupIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: pid_t) -> Result<Self, Self::Error>
	{
		if likely!(value > 0)
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(value)}))
		}
		else
		{
			Err(ParseNumberError::TooSmall)
		}
	}
}

impl From<NonZeroI32> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl Into<NonZeroI32> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl ParseNumber for ProcessGroupIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(pid) }))
		}
	}
}

impl ParseNumber for Option<ProcessGroupIdentifier>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < -1)
		{
			Err(ParseNumberError::TooShort)
		}
		// eg as in `/proc/<N>/stat`.
		else if unlikely!(pid == -1)
		{
			Ok(None)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Some(ProcessGroupIdentifier(unsafe { NonZeroI32::new_unchecked(pid) })))
		}
	}
}

impl ProcessGroupIdentifier
{
	/// Get the process group identifier (pgid) for a process identifier.
	#[inline(always)]
	pub fn process_group_identifier(process_identifier: ProcessIdentifierChoice) -> Result<Self, ()>
	{
		let result = unsafe { getpgid(process_identifier.into()) };
		if likely!(result == 0)
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(result)}))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ESRCH | EPERM => Err(()),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}

	/// Get session identifier (sid) for a process identifier.
	///
	/// The session identifier of a process is the process group identifier of the session leader.
	#[inline(always)]
	pub fn session_identifier(process_identifier: ProcessIdentifierChoice) -> Result<Self, ()>
	{
		let result = unsafe { getsid(process_identifier.into()) };
		if likely!(result == 0)
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(result)}))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ESRCH | EPERM => Err(()),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}