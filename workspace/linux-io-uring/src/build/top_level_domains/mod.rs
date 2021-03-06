// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::new_buf_writer;
use std::ffi::OsString;
use std::io;
use std::fs::File;
use std::fs::read;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;


include!("CaseFoldedLabel.rs");
include!("from_iana_comment.rs");
include!("main.rs");
include!("top_level_domains.rs");
include!("TopLevelDomainsRelativeFilePath.rs");
include!("write_case_folded_labels.rs");
include!("write_case_folded_names.rs");
