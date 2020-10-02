// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Accept Duplicate Address Detection (DAD).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion6AcceptDuplicateAddressDetection
{
	#[allow(missing_docs)]
	Disable = 0,
	
	#[allow(missing_docs)]
	Enable = 1,
	
	#[allow(missing_docs)]
	EnableAndDisableInternetProtocolVersion5IfAMediaAccessControlDuplicateLinkLocalAddressHasBeenFound = 2,
}

impl Default for InternetProtocolVersion6AcceptDuplicateAddressDetection
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6AcceptDuplicateAddressDetection::Enable
	}
}

impl InternetProtocolVersion6AcceptDuplicateAddressDetection
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion6AcceptDuplicateAddressDetection::*;
		
		match value
		{
			0 => Ok(Disable),
			
			1 => Ok(Enable),
			
			2 => Ok(EnableAndDisableInternetProtocolVersion5IfAMediaAccessControlDuplicateLinkLocalAddressHasBeenFound),
			
			_ => Err(format!("Unexpected value for accept_dad of {}", value))
		}
	}
}
