// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// High Definition Audio (HD-A) 1.0 Programming Interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum HighDefinitionAudioProgrammingInterface
{
	/// High Definition Audio (HD-A) 1.0 compatible.
	///
	/// See <http://www.intel.com/content/www/us/en/standards/standards-high-def-audio-specs-general-technology.html>.
	HighDefinitionAudioCompatible = 0x00,

	/// High Definition Audio (HD-A) 1.0 compatible with additional vendor specific extensions.
	///
	/// See <http://www.intel.com/content/www/us/en/standards/standards-high-def-audio-specs-general-technology.html>.
	HighDefinitionAudioCompatibleWithAdditionalVendorSpecificExtensions = 0x80,
}

impl HighDefinitionAudioProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::HighDefinitionAudioProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(HighDefinitionAudioCompatible),
			0x80 => Some(HighDefinitionAudioCompatibleWithAdditionalVendorSpecificExtensions),
			_ => None,
		}
	}
}
