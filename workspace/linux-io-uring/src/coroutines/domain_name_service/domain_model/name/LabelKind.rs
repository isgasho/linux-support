// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum LabelKind
{
	Bytes = 0b00,

	Extended = 0b01,

	Unallocated = 0b10,

	CompressedOffsetPointer = 0b11,
}

impl LabelKind
{
	const BytesLabelKindSize: usize = 1;
	
	const CompressedOffsetPointerLabelKindSize: usize = 2;
}
