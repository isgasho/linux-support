// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Frame headroom.
///
/// Default is `0` (`XSK_UMEM__DEFAULT_FRAME_HEADROOM`).
///
/// Must be much less than `ChunkSize`; at the very least, the maximum size is `ChunkSize - MinimumEthernetFrameSize`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FrameHeadroom(pub u32);

impl Into<usize> for FrameHeadroom
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.into_usize()
	}
}

impl FrameHeadroom
{
	/// Into usize.
	#[inline(always)]
	pub const fn into_usize(self) -> usize
	{
		self.0 as usize
	}
	
	#[inline(always)]
	pub(crate) const fn with_xdp_packet_headroom_before_frame_headroom(self) -> usize
	{
		XDP_PACKET_HEADROOM + self.into_usize()
	}
}
