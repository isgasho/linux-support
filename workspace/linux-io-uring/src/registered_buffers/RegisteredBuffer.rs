// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct RegisteredBuffer<BufferSize: MemorySize>
{
	memory_queue: ReferenceCountedLargeRingQueue<BufferSize>,
	registered_buffer_index: RegisteredBufferIndex,
}

impl<BufferSize: MemorySize> RegisteredBuffer<BufferSize>
{
	#[allow(missing_docs)]
	#[inline(always)]
	fn next_buffer_source(&self) -> Result<RegisteredBufferSource<BufferSize>, ()>
	{
		self.memory_queue.obtain_and_map(|element| RegisteredBufferSource { element, registered_buffer_index: self.registered_buffer_index }, || ())
	}
}
