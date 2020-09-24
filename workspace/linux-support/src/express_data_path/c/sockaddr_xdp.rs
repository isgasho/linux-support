// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(super) struct sockaddr_xdp
{
	/// Should always be `AF_XDP`.
	pub(crate) sxdp_family: u16,
	
	pub(crate) sxdp_flags: XdpSocketAddressFlags,
	
	pub(crate) sxdp_ifindex: NetworkInterfaceIndex,
	
	pub(crate) sxdp_queue_id: ExpressDataPathQueueIdentifier,
	
	pub(crate) sxdp_shared_umem_fd: RawFd,
}
