// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Virtual Page Frame Number (PFN).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VirtualPageFrameNumber(usize);

impl From<VirtualAddress> for VirtualPageFrameNumber
{
	#[inline(always)]
	fn from(value: VirtualAddress) -> Self
	{
		let into: usize = value.into();
		VirtualPageFrameNumber(into / PageSize::default() as usize)
	}
}

impl Into<usize> for VirtualPageFrameNumber
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl Into<u64> for VirtualPageFrameNumber
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<SeekFrom> for VirtualPageFrameNumber
{
	#[inline(always)]
	fn into(self) -> SeekFrom
	{
		let offset = self.0 * size_of::<PageMapEntry>();
		SeekFrom::Start(offset as u64)
	}
}
