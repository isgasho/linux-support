// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait OwnedRecord: Sized + Debug + DeserializeOwned + Serialize
{
	type OwnedRecords: OwnedRecords<Self>;
	
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>;
	
	fn retrieve_fixed(query_types_fixed: &QueryTypesFixed) -> Option<&Self::OwnedRecords>;
	
	#[inline(always)]
	fn store(query_types_cache: &mut QueryTypesCache, records: Self::OwnedRecords)
	{
		let query_type_cache = Self::retrieve(query_types_cache);
		
		*query_type_cache = QueryTypeCache::data(records.cache_until(), records)
	}
	
	#[inline(always)]
	fn no_data(query_types_cache: &mut QueryTypesCache, negative_cache_until: NegativeCacheUntil)
	{
		let query_type_cache = Self::retrieve(query_types_cache);
		
		*query_type_cache = QueryTypeCache::no_data(negative_cache_until);
	}
}

impl OwnedRecord for Ipv4Addr
{
	type OwnedRecords = MultipleSortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.A
	}
	
	#[inline(always)]
	fn retrieve_fixed(query_types_fixed: &QueryTypesFixed) -> Option<&Self::OwnedRecords>
	{
		query_types_fixed.A.as_ref()
	}
}

impl OwnedRecord for Ipv6Addr
{
	type OwnedRecords = MultipleSortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.AAAA
	}
	
	#[inline(always)]
	fn retrieve_fixed(query_types_fixed: &QueryTypesFixed) -> Option<&Self::OwnedRecords>
	{
		query_types_fixed.AAAA.as_ref()
	}
}
