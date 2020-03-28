// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(super) fn mbind(start: *mut c_void, len: usize, mode: i32, nodemask: *const usize, maxnode: usize, flags: MemoryBindFlags) -> isize
{
	SYS::mbind.syscall6(start as usize, len as usize, mode as usize, nodemask as usize, maxnode, flags.bits() as usize)
}
