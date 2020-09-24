// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diagnostic information.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
#[derive(Deserialize, Serialize)]
pub struct xdp_diag_umem
{
	/// Size of user memory area.
	pub size: u64,
	
	/// ?
	pub id: u32,
	
	/// Number of pages.
	///
	/// The `size` of the user memory area divided by the current `PageSize`.
	pub num_pages: u32,
	
	/// Chunk size.
	///
	/// See the trait `ChunkSize` and its implementations `AlignedChunkSize` and `UnalignedChunkSize`.
	pub chunk_size: u32,
	
	/// Frame Headroom.
	///
	/// See the struct `FrameHeadroom`.
	pub headroom: u32,
	
	/// ?Duplicates `xdp_diag_info.ifindex`.
	pub ifindex: Option<NetworkInterfaceIndex>,
	
	/// ?Duplicates `xdp_diag_info.queue_id`.
	pub queue_id: ExpressDataPathQueueIdentifier,
	
	/// `XDP_DU_F_ZEROCOPY` is the only valid flag.
	pub flags: XdpDiagnosticUserMemoryFlags,
	
	/// ?
	pub refs: u32,
}
