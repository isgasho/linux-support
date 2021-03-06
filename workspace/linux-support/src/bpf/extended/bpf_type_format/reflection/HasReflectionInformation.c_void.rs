// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl HasReflectionInformation for c_void
{
	const Type: Type = Type
	{
		type_id: TypeId::of::<c_void>(),
		size: 0,
		// See the Linux kernel function `__btf_name_by_offset()`.
		ident: "(anon)",
		data: Data::Primitive(BpfTypeFormatIntegerEncoding::Unsigned),
	};
}

pointer_type!(c_void);
