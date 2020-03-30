// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parsing of a process status file failed.
#[derive(Debug)]
pub enum ProcessStatusFileParseError
{
	/// Could not open a file.
	CouldNotOpenFile(io::Error),

	/// Could not read a line of data.
	CouldNotReadLine
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Cause.
		cause: io::Error,
	},

	/// Could not parse a line of data.
	CouldNotParseLine
	{
		/// Zero-based line number.
		zero_based_line_number: usize,

		/// Cause.
		cause: ProcessStatusStatisticParseError,
	},
}

impl Display for ProcessStatusFileParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessStatusFileParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessStatusFileParseError::*;

		match self
		{
			&CouldNotOpenFile(ref error) => Some(error),

			&CouldNotReadLine { ref cause, .. } => Some(cause),

			&CouldNotParseLine { ref cause, .. } => Some(cause),
		}
	}
}

impl From<io::Error> for ProcessStatusFileParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessStatusFileParseError::CouldNotOpenFile(error)
	}
}
