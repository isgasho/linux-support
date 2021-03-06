// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for the command `BPF_OBJ_GET_INFO_BY_FD`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandObjectGetInformationByFileDescriptor
{
	pub(crate) bpf_fd: RawFd,
	
	/// Size of data pointed to by `info`.
	pub(crate) info_len: u32,
	
	/// Pointer to data.
	pub(crate) info: AlignedU64,
}
