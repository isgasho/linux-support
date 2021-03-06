// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ResponseRecordSectionsParser<'message>
{
	now: NanosecondsSinceUnixEpoch,
	data_type: DataType,
	end_of_message_pointer: usize,
	message_header: &'message MessageHeader,
	
	parsed_names: RefCell<ParsedNames<'message>>,
	response_parsing_state: ResponseParsingState,
	duplicate_resource_record_response_parsing: DuplicateResourceRecordResponseParsing<'message>,

	query_name: &'message EfficientCaseFoldedName,
}

impl<'message> ResponseRecordSectionsParser<'message>
{
	#[inline(always)]
	pub(crate) fn new(now: NanosecondsSinceUnixEpoch, data_type: DataType, end_of_message_pointer: usize, message_header: &'message MessageHeader, parsed_names: ParsedNames<'message>, query_name: &'message EfficientCaseFoldedName) -> Self
	{
		Self
		{
			now,
			data_type,
			end_of_message_pointer,
			message_header,
			
			parsed_names: RefCell::new(parsed_names),
			response_parsing_state: ResponseParsingState::new(),
			duplicate_resource_record_response_parsing: DuplicateResourceRecordResponseParsing::default(),
			
			query_name,
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_answer_authority_and_additional_sections<RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToParsedRecords<'message, PR>>, PR: ParsedRecord>(&mut self, next_resource_record_pointer: usize, authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither, rcode_lower_4_bits: RCodeLower4Bits, answer_section_resource_record_visitor: RRV) -> Result<(usize, Answer<PR>, CanonicalNameChain<'message>, DelegationNames), SectionError<RRV::Error>>
	{
		let (next_resource_record_pointer, records, canonical_name_chain) = self.parse_answer_section(next_resource_record_pointer, answer_section_resource_record_visitor)?;
		
		let (next_resource_record_pointer, authority_resource_record_visitor) = self.parse_authority_section(next_resource_record_pointer, canonical_name_chain)?;

		let (next_resource_record_pointer, answer_existence) = self.parse_additional_section(next_resource_record_pointer, authoritative_or_authenticated_or_neither, rcode_lower_4_bits)?;
		
		let (answer, canonical_name_chain) = authority_resource_record_visitor.answer(answer_existence, records, self.now)?;
		
		Ok((next_resource_record_pointer, answer, canonical_name_records, delegation_names))
	}

	#[inline(always)]
	fn parse_answer_section<RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToParsedRecords<'message, PR>>, PR: ParsedRecord>(&mut self, next_resource_record_pointer: usize, answer_section_resource_record_visitor: RRV) -> Result<(usize, Option<OwnerNameToParsedRecordsValue<PR>>, CanonicalNameChain<'message>), AnswerSectionError<WrappingCanonicalChainError<RRV::Error>>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_answer_section();

		let mut resource_record_visitor = CanonicalNameChainAnswerSectionResourceRecordVisitor::new(answer_section_resource_record_visitor, self.query_name);
		
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AnswerSectionError::ResourceRecordsOverflowSection, |resource_record| resource_record.parse_answer_section_resource_record_in_response(self.now, self.data_type, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing))?;
		
		let (records, canonical_name_chain) = resource_record_visitor.finished();
		
		let records = records.filter(canonical_name_chain.most_canonical_name());
		
		Ok((next_resource_record_pointer, records, canonical_name_chain))
	}

	#[inline(always)]
	fn parse_authority_section(&mut self, next_resource_record_pointer: usize, canonical_name_chain: CanonicalNameChain<'message>) -> Result<(usize, AuthorityResourceRecordVisitor<'message>), AuthoritySectionError<AuthorityError>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_authority_records_section();

		let mut authority_resource_record_visitor = AuthorityResourceRecordVisitor::new(canonical_name_chain);
		
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AuthoritySectionError::ResourceRecordsOverflowSection, |resource_record| resource_record.parse_authority_section_resource_record_in_response(self.now, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut authority_resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing))?;
		
		Ok((next_resource_record_pointer, authority_resource_record_visitor.finished()))
	}

	#[inline(always)]
	fn parse_additional_section(&mut self, next_resource_record_pointer: usize, authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither, rcode_lower_4_bits: RCodeLower4Bits) -> Result<(usize, AnswerExistence), AdditionalSectionError<Infallible>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_additional_records_section();
		
		let mut discarding_resource_record_visitor = DiscardingResourceRecordVisitor::default();
		
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AdditionalSectionError::ResourceRecordsOverflowSection, |resource_record| resource_record.parse_additional_section_resource_record_in_response(self.now, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut discarding_resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing, authoritative_or_authenticated_or_neither, rcode_lower_4_bits))?;
		
		discarding_resource_record_visitor.finished();
		let answer_existence = self.response_parsing_state.parse_extended_dns_outcome::<Infallible>()?;
		
		Ok((next_resource_record_pointer, answer_existence))
	}
	
	#[inline(always)]
	fn loop_over_resource_records<E: error::Error>(&self, mut next_resource_record_pointer: usize, number_of_resource_records: u16, overflow_section_error: E, mut parse_method: impl FnMut(&'message ResourceRecord) -> Result<usize, E>) -> Result<usize, E>
	{
		for _ in 0 .. number_of_resource_records
		{
			if unlikely!(next_resource_record_pointer == self.end_of_message_pointer)
			{
				return Err(overflow_section_error)
			}
			let resource_record = next_resource_record_pointer.unsafe_cast::<ResourceRecord>();
			next_resource_record_pointer = parse_method(resource_record)?;
		}
		Ok(next_resource_record_pointer)
	}
}
