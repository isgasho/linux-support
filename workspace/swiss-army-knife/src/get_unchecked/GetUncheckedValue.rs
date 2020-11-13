// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
pub trait GetUncheckedValue<T: Copy>: GetUnchecked<T>
{
	/// Like `get_unchecked()`, but, if debug assertions are configured, checks and panics.
	fn get_unchecked_value_safe<AUI: AsUsizeIndex>(&self, index: AUI) -> T
	{
		* self.get_unchecked_safe(index)
	}
}
