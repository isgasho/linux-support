// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global configuration error kind.
#[derive(Debug)]
pub enum GlobalConfigurationError
{
	#[allow(missing_docs)]
	GlobalSchedulingConfiguration(GlobalSchedulingConfigurationError),

	#[allow(missing_docs)]
	GlobalPipeConfiguration(GlobalPipeConfigurationError),

	#[allow(missing_docs)]
	GlobalFileLeasingConfiguration(GlobalFileLeasingConfigurationError),

	#[allow(missing_docs)]
	GlobalPosixMessageQueueConfiguration(GlobalPosixMessageQueueConfigurationError),

	#[allow(missing_docs)]
	GlobalSystemVMessageQueueConfiguration(GlobalSystemVMessageQueueConfigurationError),

	#[allow(missing_docs)]
	GlobalInotifyConfiguration(GlobalInotifyConfigurationError),

	#[allow(missing_docs)]
	GlobalEPollConfiguration(GlobalEPollConfigurationError),

	#[allow(missing_docs)]
	GlobalLinuxKernelAsynchronousIoConfiguration(GlobalLinuxKernelAsynchronousIoConfigurationError),

	#[allow(missing_docs)]
	GlobalFileHandleConfiguration(GlobalFileHandleConfigurationError),

	#[allow(missing_docs)]
	GlobalFileDescriptorConfiguration(GlobalFileDescriptorConfigurationError),
}

impl Display for GlobalConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalConfigurationError::*;

		match self
		{
			&GlobalSchedulingConfiguration(ref cause) => Some(error),

			&GlobalPipeConfiguration(ref cause) => Some(error),

			&GlobalFileLeasingConfiguration(ref cause) => Some(error),

			&GlobalPosixMessageQueueConfiguration(ref cause) => Some(error),

			&GlobalSystemVMessageQueueConfiguration(ref cause) => Some(error),

			&GlobalInotifyConfiguration(ref cause) => Some(error),

			&GlobalEPollConfiguration(ref cause) => Some(error),

			&GlobalLinuxKernelAsynchronousIoConfiguration(ref cause) => Some(error),

			&GlobalFileHandleConfiguration(ref cause) => Some(error),

			&GlobalFileDescriptorConfiguration(ref cause) => Some(error),
		}
	}
}

impl From<GlobalSchedulingConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSchedulingConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSchedulingConfiguration(cause)
	}
}

impl From<GlobalPipeConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalPipeConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalPipeConfiguration(cause)
	}
}

impl From<GlobalFileLeasingConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileLeasingConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileLeasingConfiguration(cause)
	}
}

impl From<GlobalPosixMessageQueueConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalPosixMessageQueueConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalPosixMessageQueueConfiguration(cause)
	}
}

impl From<GlobalSystemVMessageQueueConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSystemVMessageQueueConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSystemVMessageQueueConfiguration(cause)
	}
}

impl From<GlobalInotifyConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalInotifyConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalInotifyConfiguration(cause)
	}
}

impl From<GlobalEPollConfiguration> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalEPollConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalEPollConfiguration(cause)
	}
}

impl From<GlobalLinuxKernelAsynchronousIoConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxKernelAsynchronousIoConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalLinuxKernelAsynchronousIoConfiguration(cause)
	}
}

impl From<GlobalFileHandleConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileHandleConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileHandleConfiguration(cause)
	}
}

impl From<GlobalFileDescriptorConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileDescriptorConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileDescriptorConfiguration(cause)
	}
}
