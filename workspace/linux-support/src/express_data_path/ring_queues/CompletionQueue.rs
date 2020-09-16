// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used in conjunction with the `TransmitQueue`.
pub(super) type CompletionQueue = XskRingQueue<ConsumerXskRingQueueKind, UmemDescriptor>;

impl CompletionQueue
{
	#[inline(always)]
	pub(super) fn from_completion_memory_map_offsets(user_memory_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, memory_map_offsets: &xdp_mmap_offsets, completion_ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::from_ring_queue_offsets(user_memory_socket_file_descriptor, memory_map_offsets.completion_ring_offsets(), completion_ring_queue_depth, defaults, XDP_UMEM_PGOFF_COMPLETION_RING)
	}
	
	/// Based on `xsk_ring_cons__comp_addr()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn completion_adddress(&self, index: u32) -> &UmemDescriptor
	{
		self.ring_entry(index)
	}
}
