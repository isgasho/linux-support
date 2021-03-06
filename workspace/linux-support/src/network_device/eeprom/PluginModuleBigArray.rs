// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
mod missing_docs_hack_because_big_array_does_not_allow_doc_comments
{
	use super::*;
	
	big_array!
	{
		PluginModuleBigArray;
		256, 640,
	}
}

pub use missing_docs_hack_because_big_array_does_not_allow_doc_comments::PluginModuleBigArray;
