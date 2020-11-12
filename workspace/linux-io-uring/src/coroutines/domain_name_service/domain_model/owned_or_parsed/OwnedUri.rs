// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owned URI.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OwnedUri(URI<'static>);

impl HasTypeEquality for OwnedUri
{
	type TypeEquality = OwnedTypeEquality;
}

impl OwnedOrParsedUri for OwnedUri
{
}

impl Deref for OwnedUri
{
	type Target = URI<'static>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'message> From<ParsedUri<'message>> for OwnedUri
{
	#[inline(always)]
	fn from(value: ParsedUri<'message>) -> Self
	{
		Self(value.into_owned())
	}
}
