// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use errno::errno;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::{io, error};
use std::path::PathBuf;
use crate::user_and_groups::assert_effective_user_id_is_root;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;


include!("ResourceName.rs");
include!("ResourceLimit.rs");
include!("ResourceLimitError.rs");
include!("ResourceLimitsSet.rs");
include!("SoftAndHardResourceLimit.rs");
