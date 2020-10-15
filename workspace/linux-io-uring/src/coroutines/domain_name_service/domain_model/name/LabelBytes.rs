// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Label bytes in US-ASCII, but casing may be mixed and invalid characters may be present.
///
/// Maximum length is 63.
///
/// If empty (length of 0) then this represents the Root, terminal label.
pub type LabelBytes<'message> = &'message [u8];
