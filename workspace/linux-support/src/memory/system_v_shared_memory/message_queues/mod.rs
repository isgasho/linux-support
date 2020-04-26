// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use std::io;
use std::num::NonZeroU32;


include!("maximum_message_size.rs");
include!("maximum_number_of_queue_identifiers.rs");
include!("maximum_queue_size_in_bytes.rs");
include!("set_maximum_message_size.rs");
include!("set_maximum_number_of_queue_identifiers.rs");
include!("set_maximum_queue_size_in_bytes.rs");
