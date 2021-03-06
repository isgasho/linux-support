// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ResponseParsingState
{
	number_of_cname_records_in_answer_section: Cell<usize>,
	number_of_dname_records_in_answer_section: Cell<usize>,
	have_yet_to_see_a_soa_resource_record: Cell<bool>,
	have_yet_to_see_an_edns_opt_resource_record: Cell<Option<AnswerExistence>>,
}

impl ResponseParsingState
{
	#[inline(always)]
	const fn new() -> Self
	{
		Self
		{
			number_of_cname_records_in_answer_section: Cell::new(0),
			number_of_dname_records_in_answer_section: Cell::new(0),
			have_yet_to_see_a_soa_resource_record: Cell::new(true),
			have_yet_to_see_an_edns_opt_resource_record: Cell::new(None),
		}
	}
	
	#[inline(always)]
	pub(crate) fn validate_only_one_CNAME_record_in_answer_section_when_query_type_was_CNAME<E: error::Error>(&self) -> Result<(), AnswerSectionError<E>>
	{
		let number_of_cname_records_in_answer_section = self.number_of_cname_records_in_answer_section.get();
		if unlikely!(number_of_cname_records_in_answer_section > 1)
		{
			Err(AnswerSectionError::MoreThanOneCNAMERecordIsNotValidInAnswerSectionForACNAMEQuery)
		}
		else
		{
			self.number_of_cname_records_in_answer_section.set(number_of_cname_records_in_answer_section + 1);
			Ok(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn validate_only_one_DNAME_record_in_answer_section_when_query_type_was_DNAME<E: error::Error>(&self) -> Result<(), AnswerSectionError<E>>
	{
		let number_of_dname_records_in_answer_section = self.number_of_dname_records_in_answer_section.get();
		if unlikely!(number_of_dname_records_in_answer_section > 1)
		{
			Err(AnswerSectionError::MoreThanOneDNAMERecordIsNotValidInAnswerSectionForADNAMEQuery)
		}
		else
		{
			self.number_of_dname_records_in_answer_section.set(number_of_dname_records_in_answer_section + 1);
			Ok(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn parsing_a_soa_record(&self) -> Result<(), SOAHandleRecordTypeError>
	{
		if likely!(self.have_yet_to_see_a_soa_resource_record())
		{
			self.have_yet_to_see_a_soa_resource_record.set(false);
			Ok(())
		}
		else
		{
			Err(SOAHandleRecordTypeError::MoreThanOneStartOfAuthorityResourceRecordInTheAnswerAndAdditionalSectionsCombined)
		}
	}
	
	#[inline(always)]
	pub(crate) fn parsed_an_edns_opt_record(&self, answer_existence: AnswerExistence) -> Result<(), ExtendedDnsError>
	{
		if likely!(self.have_yet_to_see_an_edns_opt_resource_record.get().is_none())
		{
			self.have_yet_to_see_an_edns_opt_resource_record.set(Some(answer_existence));
			Ok(())
		}
		else
		{
			Err(ExtendedDnsError::MoreThanOneExtendedDnsOptResourceRecord)
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_extended_dns_outcome<E: error::Error>(&self) -> Result<AnswerExistence, AdditionalSectionError<E>>
	{
		self.have_yet_to_see_an_edns_opt_resource_record.get().ok_or(AdditionalSectionError::ResponseDidNotContainAnExtendedDnsOptMetaResourceRecord)
	}
	
	#[inline(always)]
	fn have_yet_to_see_a_soa_resource_record(&self) -> bool
	{
		self.have_yet_to_see_a_soa_resource_record.get()
	}
}
