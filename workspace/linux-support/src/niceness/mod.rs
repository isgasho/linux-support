// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::*;
use crate::process::*;
use crate::strings::Radix;
use crate::strings::parse_number::*;
use errno::errno;
use libc::*;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::ErrorKind;
use std::mem::transmute;
use std::num::NonZeroU8;
use std::path::PathBuf;
use std::convert::TryFrom;


include!("Nice.rs");
include!("ProcessNiceness.rs");
include!("ProcessNicenessAdjustmentError.rs");
include!("RealTimePriority.rs");
