// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Host name.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LinuxKernelHostName(Box<[u8]>);

impl Deref for LinuxKernelHostName
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl LinuxKernelHostName
{
	/// New instance.
	#[inline(always)]
	pub fn new(proc_path: &ProcPath) -> io::Result<Option<Self>>
	{
		let host_name = proc_path.sys_kernel_file_path("hostname").read_raw_without_line_feed()?;
		if unlikely!(&host_name[..] == b"(none)")
		{
			Ok(None)
		}
		else
		{
			Ok(Some(Self(host_name)))
		}
	}
}
