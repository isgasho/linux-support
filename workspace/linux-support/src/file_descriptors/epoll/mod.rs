// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use super::file_descriptor_information::*;
use self::c::*;
use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;


/// System call and libc wrapping of system call specific details.
pub mod c;


include!("EPollAddError.rs");
include!("EPollAddFlags.rs");
include!("EPollEventFlags.rs");
include!("EPollFileDescriptor.rs");
include!("EPollInformationItem.rs");
include!("EPollModifyError.rs");
include!("EPollModifyFlags.rs");
include!("EPollTimeOut.rs");
include!("EPollWaitError.rs");
include!("maximum_number_of_watched_file_descriptors_per_user.rs");
include!("set_maximum_number_of_watched_file_descriptors_per_user.rs");
