// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct Records<'cache, Record: Sized + Debug>(HashMap<CaseFoldedName<'cache>, Present<Record>>);

impl<'cache, Record: Sized + Debug> Clone for Records<'cache, Record>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
}

impl<'cache, Record: Sized + Debug> Into<HashMap<CaseFoldedName<'cache>, Present<Record>>> for Records<'cache, Record>
{
	#[inline(always)]
	fn into(self) -> HashMap<CaseFoldedName<'cache>, Present<Record>>
	{
		self.0
	}
}

impl<'cache, Record: Sized + Debug> Records<'cache, Record>
{
	#[inline(always)]
	pub(crate) fn with_capacity(capacity: usize) -> Self
	{
		Self(HashMap::with_capacity(capacity))
	}
	
	#[inline(always)]
	pub(crate) fn has_records(&self) -> bool
	{
		!self.0.is_empty()
	}
	
	#[inline(always)]
	pub(crate) fn store_unprioritized_and_unweighted<'message>(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Record)
	where 'cache: 'message
	{
		self.store_unweighted(name, cache_until, Priority::Unassigned, record)
	}
	
	#[inline(always)]
	pub(crate) fn store_unweighted<'message>(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, priority_or_preference: Priority, record: Record)
	where 'cache: 'message
	{
		self.store(name, cache_until, priority_or_preference, Weight::Unassigned, record)
	}
	
	#[inline(always)]
	pub(crate) fn store<'message>(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, priority: Priority, weight: Weight, record: Record)
	where 'cache: 'message
	{
		let present = self.0.entry(CaseFoldedName::map(name)).or_insert_with(Present::default);
		
		let priority_to_sorted_weighted_records_map = match cache_until
		{
			None => &mut present.use_once,
			
			Some(cache_until) => present.cached.entry(cache_until).or_insert_with(PriorityToSortedWeightedRecordsMap::default),
		};
		priority_to_sorted_weighted_records_map.insert(priority, weight, record);
	}
	
}
