// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum Transmission Unit (MTU).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumTransmissionUnit(pub u32);

impl Into<usize> for MaximumTransmissionUnit
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Into<i32> for MaximumTransmissionUnit
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0 as i32
	}
}

impl MaximumTransmissionUnit
{
	/// Ethernet inclusive minimum.
	pub const EthernetInclusiveMinimumIncludingFrameCheckSequence: Self = Self(68);
}
