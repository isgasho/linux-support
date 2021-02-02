// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union uffd_msg_arg_pagefault_feat
{
	/// Will only be populated if the `Features::ThreadIdentifier` is enabled.
	///
	/// Linux specified this is `u32` but it would seem to be `pid_t`.
	pub(super) ptid: Option<ThreadIdentifier>,
}
