// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
#[doc(hidden)]
pub struct HiddenChildProcessArguments<CPA>
{
	send_pipe_file_descriptor: SendPipeFileDescriptor,
	receive_pipe_file_descriptor: ReceivePipeFileDescriptor,
	new_root: PathBuf,
	child_process_argument: CPA,
}

impl<CPA> HiddenChildProcessArguments<CPA>
{
	#[inline(always)]
	pub(crate) fn unbox(self: Box<Self>) -> Self
	{
		*self
	}
}
