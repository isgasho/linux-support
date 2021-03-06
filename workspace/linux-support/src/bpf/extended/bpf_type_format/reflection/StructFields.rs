// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// A set of fields associated with a type or `enum` variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructFields
{
	/// A set of named fields.
	Named(&'static [NamedField]),
	
	/// A set of index-addressed fields.
	Unnamed(&'static [UnnamedField]),
	
	/// The empty set of fields, applicable to unit structs or enum variants.
	Unit,
}
