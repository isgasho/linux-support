// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


pub(crate) mod reference_identifier;


include!("AssociationMode.rs");
include!("AssociationModeAndVersionNumberAndLeapIndicator.rs");
include!("KeyIdentifier.rs");
include!("LeapIndicator.rs");
include!("MessageDigest.rs");
include!("SignedShortFormat.rs");
include!("Stratum.rs");
include!("UnsignedShortFormat.rs");
include!("UnsignedTimestampFormat.rs");
include!("VersionNumber.rs");
