// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Are leases enabled?
///
/// Reads from `/proc/sys/fs/leases-enable`.
///
/// Default is `true`
#[inline(always)]
pub fn leases_enabled(proc_path: &ProcPath) -> bool
{
	proc_path.sys_fs_file_path("leases-enable").read_zero_or_one_bool().unwrap()
}
