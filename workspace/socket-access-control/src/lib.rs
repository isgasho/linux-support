// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]


//! #socket-access-control
//!
//! Socket access control.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::longest_prefix_match_table::*;
use linux_support::file_descriptors::socket::*;
use linux_support::file_descriptors::socket::c::*;
use linux_support::internet_protocol::*;
use linux_support::user_and_groups::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::num::NonZeroU8;
use std::ops::Deref;
use std::rc::Rc;


/// A longest prefix match table.
pub mod longest_prefix_match_table;


include!("AccessControl.rs");
include!("RemotePeerAddressBasedAccessControl.rs");
