// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Zero page mode.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	struct ZeroPageMode: u64
	{
		/// Do not wake up.
		const DoNotWakeUp = UFFDIO_ZEROPAGE_MODE_DONTWAKE;
	}
}

impl Default for ZeroPageMode
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl ZeroPageMode
{
	const fn new(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool) -> Self
	{
		unsafe { ZeroPageMode::from_bits_unchecked((!wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange) as u64) }
	}
}
