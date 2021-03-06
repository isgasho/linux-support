// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum QueryTypeCacheEntry<'cache, Record: Record>
{
	/// One-time use.
	AbsentUseOnce(Rc<StartOfAuthority<EfficientCaseFoldedName>>),
	
	/// Negatively cached.
	AbsentNegativelyCached(NanosecondsSinceUnixEpoch, Rc<StartOfAuthority<EfficientCaseFoldedName>>),
	
	Present(PresentMultiple<Record>),
}

impl<'cache, Record: Record> LeastRecentlyUsedCacheValue for QueryTypeCacheEntry<'cache, Record>
{
	#[inline(always)]
	fn records_count(&self) -> NonZeroUsize
	{
		use self::QueryTypeCacheEntry::*;
		
		const One: NonZeroUsize = new_non_zero_usize(1);
		
		match self
		{
			AbsentUseOnce(_) | AbsentNegativelyCached(_, _) => One,
			
			Present(ref present) => present.records_count(),
		}
	}
}
