// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Variable linkage.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum BpfTypeFormatVariableLinkage
{
	/// `BTF_VAR_STATIC`.
	Static = 0,
	
	/// `BTF_VAR_GLOBAL_ALLOCATED`.
	GlobalAllocated = 1,
	
	/// `BTF_VAR_GLOBAL_EXTERN`.
	GlobalExtern = 2,
}

impl Default for BpfTypeFormatVariableLinkage
{
	#[inline(always)]
	fn default() -> Self
	{
		BpfTypeFormatVariableLinkage::Static
	}
}
