// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Flow specification for TCP/IPv6, UDP/IPv6 or SCTP/IPv6.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct ethtool_tcpip6_spec
{
	/// Source host.
	pub(crate) ip6src: [BigEndianU32; 4],
	
	/// Destination host.
	pub(crate) ip6dst: [BigEndianU32; 4],
	
	/// Source port.
	pub(crate) psrc: BigEndianU16,
	
	/// Destination port.
	pub(crate) pdst: BigEndianU16,
	
	/// Traffic Class (TC).
	pub(crate) tclass: u8,
}

impl FlowSpecification for ethtool_tcpip6_spec
{
}

impl CommonLayer3FlowSpecification<[BigEndianU32; 4]> for ethtool_tcpip6_spec
{
	#[inline(always)]
	fn source_address(&self) -> [BigEndianU32; 4]
	{
		self.ip6src
	}
	
	#[inline(always)]
	fn destination_address(&self) -> [BigEndianU32; 4]
	{
		self.ip6dst
	}
	
	#[inline(always)]
	fn tos_or_tclass(&self) -> u8
	{
		self.tclass
	}
}

impl CommonLayer4FlowSpecification<[BigEndianU32; 4]> for ethtool_tcpip6_spec
{
	#[inline(always)]
	fn source_port(&self) -> BigEndianU16
	{
		self.psrc
	}
	
	#[inline(always)]
	fn destination_port(&self) -> BigEndianU16
	{
		self.pdst
	}
}
