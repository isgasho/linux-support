// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::file_descriptors::AsRawFdExt;
use super::*;
use super::file::*;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::memory::huge_pages::HugePageSize;
use crate::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use crate::memory::huge_pages::PageSizeOrHugePageSize;
use crate::vectors::VectoredRead;
use crate::vectors::VectoredWrite;
use std::fmt::Arguments;
use std::io::IoSlice;
use std::io::IoSliceMut;
use std::io::SeekFrom;


include!("FileSeals.rs");include!("MemoryFileDescriptor.rs");
