// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A common process configuration execution error.
#[derive(Debug)]
pub enum DetailedProcessConfigurationError
{
	/// Process niceness adjustment failed.
	ProcessNiceAdjustmentFailed(ProcessNiceConfigurationError),

	/// Process affinity setting failed.
	CouldNotSetCurrentProcessAffinity(io::Error),

	/// Could not load seccomp filters.
	CouldNotLoadSeccompFilters,
}

impl Display for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DetailedProcessConfigurationError::*;

		match self
		{
			&ProcessNiceAdjustmentFailed(ref error) => Some(error),

			&CouldNotSetCurrentProcessAffinity(ref error) => Some(error),

			&CouldNotLoadSeccompFilters => None,
		}
	}
}

impl From<ProcessNiceConfigurationError> for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn from(error: ProcessNiceConfigurationError) -> Self
	{
		DetailedProcessConfigurationError::ProcessNiceAdjustmentFailed(error)
	}
}

impl From<io::Error> for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		DetailedProcessConfigurationError::CouldNotSetCurrentProcessAffinity(error)
	}
}
