// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Non-volatile memory subsystem Programming Interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum NonVolatileMemorySubsystemProgrammingInterface
{
	/// Vendor-specific.
	VendorSpecific = 0x00,

	/// Non-Volatile Memory Host Controller Interface.
	NvmHci = 0x01,

	/// NVM Express interface.
	NvmExpress = 0x02,
}

impl NonVolatileMemorySubsystemProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::NonVolatileMemorySubsystemProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(VendorSpecific),
			0x01 => Some(NvmHci),
			0x02 => Some(NvmExpress),
			_ => None,
		}
	}
}
