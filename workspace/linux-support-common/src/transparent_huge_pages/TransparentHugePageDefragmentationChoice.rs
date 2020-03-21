// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transparent Huge Page (THP) defragmentation choice.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransparentHugePageDefragmentationChoice
{
	/// Never defragment.
	Never,

	/// Defer defragmentation until allocation requires it.
	Defer,

	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	Advise,

	/// Like `Defer` and `MAdvise`.
	///
	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	DeferAndAdvise,
}

impl TransparentHugePageDefragmentationChoice
{
	/// To value.
	#[inline(always)]
	pub fn to_value(self) -> &'static str
	{
		use self::TransparentHugePageDefragmentationChoice::*;

		match self
		{
			Never => "never",
			Defer => "defer",
			Advise => "madvise",
			DeferAndAdvise => "defer+madvise",
		}
	}

	/// Defrag value.
	#[inline(always)]
	pub fn defrag_value(self) -> &'static str
	{
		use self::TransparentHugePageDefragmentationChoice::*;

		match self
		{
			Never => "0",
			_ => "1",
		}
	}
}
