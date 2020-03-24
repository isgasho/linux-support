// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// System timer.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum SystemTimerGenericSystemPeripheralProgrammingInterface
{
	/// Generic 8259.
	Generic8254 = 0x00,

	/// ISA.
	Isa = 0x01,

	/// EISA (two timers).
	Eisa = 0x02,

	/// High Performance Event Timer (HPET).
	HighPerformanceEventTimer = 0x03,
}

impl SystemTimerGenericSystemPeripheralProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::SystemTimerGenericSystemPeripheralProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(Generic8254),
			0x01 => Some(Isa),
			0x02 => Some(Eisa),
			0x03 => Some(HighPerformanceEventTimer),
			_ => None,
		}
	}
}
