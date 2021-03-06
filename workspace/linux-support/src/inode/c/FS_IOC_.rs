// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const FS_IOC_GETFLAGS: i32 = 2148034049u32 as i32;
pub(crate) const FS_IOC_SETFLAGS: i32 = 1074292226u32 as i32;
pub(crate) const FS_IOC_GETVERSION: i32 = 2148038145u32 as i32;
pub(crate) const FS_IOC_SETVERSION: i32 = 1074296322u32 as i32;
pub(crate) const FS_IOC_FIEMAP: i32 = 3223348747u32 as i32;

/*
#define FS_IOC_FSGETXATTR		_IOR('X', 31, struct fsxattr)
#define FS_IOC_FSSETXATTR		_IOW('X', 32, struct fsxattr)
#define FS_IOC_GETFSLABEL		_IOR(0x94, 49, char[FSLABEL_MAX])
#define FS_IOC_SETFSLABEL		_IOW(0x94, 50, char[FSLABEL_MAX])
*/
