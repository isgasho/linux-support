// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A 16-bit message identifier, set in a request and returned in a reply.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MessageIdentifier(BigEndianU16);

impl MessageIdentifier
{
	/// Random.
	#[inline(always)]
	pub(crate) fn random() -> Self
	{
		Self(fast_slightly_insecure_random_u16().to_be_bytes())
	}
}

