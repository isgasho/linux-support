// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


extern "C"
{
	/// In practice, this function's varargs (`...`) are fixed as: `ptid: *mut pid_t, newtls: *mut c_void, ctid: *mut pid_t`.
	fn clone(func: *mut c_void, child_stack: *mut c_void, flags: c_int, arg: *mut c_void, ...) -> c_int;
}
