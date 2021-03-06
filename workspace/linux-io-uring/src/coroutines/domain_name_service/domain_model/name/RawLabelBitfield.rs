// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RawLabelBitfield(u8);

impl RawLabelBitfield
{
	#[inline(always)]
	const fn is_root(self) -> bool
	{
		self.0 == 0x00
	}

	#[inline(always)]
	const fn raw_kind(self) -> LabelKind
	{
		unsafe { transmute(self.0 >> 6) }
	}

	#[inline(always)]
	const fn bottom_6_bits_as_usize(self) -> usize
	{
		self.bottom_6_bits_as_u8() as usize
	}
	
	#[inline(always)]
	const fn bottom_6_bits_as_u8(self) -> u8
	{
		self.0 & 0b0011_1111
	}
}
