// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Waits a maximum of one millisecond for signals.
#[inline(always)]
pub fn one_millisecond_timed_wait_for_signals(signals_to_wait_for: &sigset_t) -> TimedSignalWait
{
	const OneMillisecondTimeOut: timespec = timespec
	{
		tv_sec: 0,
		tv_nsec:
		{
			const OneMillisecondInNanoseconds: i64 = 1_000_000;
			OneMillisecondInNanoseconds
		},
	};

	let result = unsafe { sigtimedwait(signals_to_wait_for, null_mut(), &OneMillisecondTimeOut) };

	use self::TimedSignalWait::*;

	match result
	{
		signal if signal >= 0 => Signalled(signal),

		-1 => match errno().0
		{
			EAGAIN => TimedOut,
			EINTR => OtherSignalInterrupted,
			EINVAL => panic!("timespec specified a tv_nsec value less than zero or greater than or equal to 1000 million"),

			error_number @ _ => panic!("Invalid error number '{}' from sigtimedwait()", error_number),
		},

		illegal @ _ => panic!("Illegal result '{}' from sigtimedwait()", illegal)
	}
}
