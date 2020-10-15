// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! iterator_next_label
{
	($self: ident) =>
	{
		{
			let pointer_to_label = $self.pointer_to_label;
			let label = Label::label(pointer_to_label);

			if unlikely!(label.is_root())
			{
				return None
			}

			(label, pointer_to_label)
		}
	}
}
