// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A SHA-2 512 digest.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sha2_512(Array512Bits);

impl Deref for Sha2_512
{
	type Target = Array512Bits;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AsRef<[u8]> for Sha2_512
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.0.as_ref()
	}
}

impl Borrow<[u8]> for Sha2_512
{
	#[inline(always)]
	fn borrow(&self) -> &[u8]
	{
		self.0.borrow()
	}
}

impl Digest for Sha2_512
{
	type Array = Array512Bits;
	
	const DigestSizeInBits: usize = 512;
	
	#[inline(always)]
	unsafe fn new_unchecked<'message>(digest_data: &'message [u8]) -> &'message Self
	{
		let digest = &* (digest_data as *const Self::Array);
		transmute (digest_data as &Self)
	}
}
