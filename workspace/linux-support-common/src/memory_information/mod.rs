// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::huge_pages::HugePageSize;
use crate::paths::*;
use std::collections::HashMap;
use crate::strings::splitn;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::path::Path;
use std::str::from_utf8;
use std::str::Utf8Error;


include!("MemoryInformation.rs");include!("MemoryInformationName.rs");include!("MemoryInformationParseError.rs");
include!("MemoryInformationUnit.rs");
include!("VirtualMemoryStatisticName.rs");