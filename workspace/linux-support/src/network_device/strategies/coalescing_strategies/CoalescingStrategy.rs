// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) trait CoalescingStrategy
{
	fn coalesce_configuration(&self, paired_transmit_receive_queues: bool, receive_coalescing_preference: &ReceiveCoalescingPreference, transmit_coalescing_preference: &TransmitCoalescingPreference) -> CoalesceConfiguration;
}
