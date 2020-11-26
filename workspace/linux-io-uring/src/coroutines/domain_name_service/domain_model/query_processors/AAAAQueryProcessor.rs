// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AAAAQueryProcessor;

impl QueryProcessor for AAAAQueryProcessor
{
	const DT: DataType = DataType::AAAA;
	
	type PR<'message> = Ipv6Addr;
	
	type RRV<'message> = AAAAQueryProcessorResourceRecordVisitor<'message>;
	
	fn new<'message>(query_name: &'message FullyQualifiedDomainName) -> Self::RRV<'message>
	{
		AAAAQueryProcessorResourceRecordVisitor
		{
			query_name,
			records: OwnerNameToRecords::default(),
		}
	}
	
	#[inline(always)]
	fn store_records_in_query_types_cache<'message>(query_types_cache: &mut QueryTypesCache, records: OwnerNameToRecordsValue<Self::PR<'message>>)
	{
		query_types_cache.store_AAAA(records)
	}
}
