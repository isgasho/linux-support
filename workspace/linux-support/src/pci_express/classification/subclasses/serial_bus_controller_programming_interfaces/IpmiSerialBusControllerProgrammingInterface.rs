// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// IPMI.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum IpmiSerialBusControllerProgrammingInterface
{
	/// SMIC Interface.
	Smic = 0x00,

	/// Keyboard Controller Style Interface.
	KeyboardControllerStyle = 0x01,

	/// Block Transfer Interface.
	BlockTransfer = 0x02,
}

impl IpmiSerialBusControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::IpmiSerialBusControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(Smic),
			0x01 => Some(KeyboardControllerStyle),
			0x02 => Some(BlockTransfer),
			_ => None,
		}
	}
}
