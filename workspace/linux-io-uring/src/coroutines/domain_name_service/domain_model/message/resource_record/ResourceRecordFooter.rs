// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, packed)]
struct ResourceRecordFooter
{
	type_: DataType,
	
	class: BigEndianU16,
	
	ttl: BigEndianI32,
	
	rdlen: BigEndianU16,
	
	rdata: ResourceData,
}

impl ResourceRecordFooter
{
	const TypeSize: usize = size_of::<DataType>();
	const ClassSize: usize = size_of::<BigEndianU16>();
	const TimeToLiveSize: usize = size_of::<BigEndianI32>();
	const ResourceDataLengthSize: usize = size_of::<BigEndianU16>();
	
	const MinimumSize: usize = Self::TypeSize + Self::ClassSize + Self::TimeToLiveSize + Self::ResourceDataLengthSize + ResourceData::MinimumSize;

	#[inline(always)]
	fn resource_record_type(&self) -> DataType
	{
		self.type_
	}

	#[inline(always)]
	fn resource_record_class_is_internet(&self) -> Result<(), ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError>
	{
		let class = self.class;

		if likely!(self.class == [0x00, 0x01])
		{
			Ok(())
		}
		else
		{
			Err(ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError::ClassIsReservedUnassignedOrObsolete(self.type_, class))
		}
	}

	#[inline(always)]
	fn requestors_udp_payload_size(&self) -> u16
	{
		self.debug_assert_is_opt();

		self.class.from_network_endian_to_native_endian()
	}

	#[inline(always)]
	fn time_to_live(&self) -> TimeInSeconds
	{
		TimeInSeconds(self.ttl)
	}

	#[inline(always)]
	fn extended_response_code_and_flags(&self) -> ExtendedResponseCodeAndFlags
	{
		self.debug_assert_is_opt();

		ExtendedResponseCodeAndFlags(self.ttl)
	}

	#[inline(always)]
	fn resource_data_length(&self) -> u16
	{
		self.rdlen.from_network_endian_to_native_endian()
	}

	#[inline(always)]
	fn resource_data(&self) -> &ResourceData
	{
		&self.rdata
	}

	#[inline(always)]
	fn debug_assert_is_opt(&self)
	{
		debug_assert_eq!(self.type_.0 , MetaType::OPT.0, "This is not an EDNS0 extension record")
	}
}
