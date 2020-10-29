// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessorResourceRecordVisitor<'cache: 'message, 'message>
{
	query_name: &'message CaseFoldedName<'cache>,
	present: Present<CaseFoldedName<'cache>>,
}

impl<'cache: 'message, 'message> ResourceRecordVisitor<'message> for MXQueryProcessorResourceRecordVisitor<'cache, 'message>
{
	type Error = Infallible;
	
	type Finished = Present<CaseFoldedName<'cache>>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.present
	}
	
	#[inline(always)]
	fn MX(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: MailExchange<'message>) -> Result<(), Self::Error>
	{
		if unlikely!(!name.eq(self.query_name))
		{
			return Ok(())
		}
		
		self.present.store_unweighted(cache_until, record.preference, CaseFoldedName::from(record.mail_server_name));
		Ok(())
	}
}
