// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A register or immediate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum RegisterOrImmediate<'de>
{
	/// Register.
	Register(Register),

	/// Immediate.
	Immediate(Immediate<'de, i32>)
}

impl<'de> From<Register> for RegisterOrImmediate<'de>
{
	#[inline(always)]
	fn from(value: Register) -> Self
	{
		RegisterOrImmediate::Register(value)
	}
}

impl<'de> From<Immediate<'de>> for RegisterOrImmediate<'de>
{
	#[inline(always)]
	fn from(value: Immediate<'de>) -> Self
	{
		RegisterOrImmediate::Immediate(value)
	}
}
