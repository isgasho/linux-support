// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A failure caused when moving a page.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum PerPageMoveError
{
	/// Can only be moved if move all specified.
	PageIsMappedByMultipleProcesses = EACCES,

	/// This occurs if a page is undergoing I/O or another kernel subsystem is holding a reference to the page.
	BusyTryAgainLater = EBUSY,

	/// This is a zero page or the memory area is not mapped by the process.
	Fault = EFAULT,

	/// Unable to write back a page.
	///
	/// The page has to be written back in order to move it since the page is dirty and the filesystem does not provide a migration function that would allow the move of dirty pages.
	CanNotWriteBackPage = EIO,

	/// A dirty page cannot be moved.
	///
	/// The filesystem does not provide a migration function and has no ability to write back pages.
	DirtyPageCanNotBeMoved = EINVAL,

	/// Not present
	PageIsNotPresent = ENOENT,

	/// Unable to allocate memory on target node.
	OutOfMemoryOnTargetNode = ENOMEM,
}

impl Display for PerPageMoveError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<PerPageMoveError as Debug>::fmt(self, f)
	}
}

impl error::Error for PerPageMoveError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}

impl PerPageMoveError
{
	fn from_i32(error_code: i32) -> Self
	{
		use self::PerPageMoveError::*;

		match error_code
		{
			EACCES => PageIsMappedByMultipleProcesses,
			EBUSY => BusyTryAgainLater,
			EFAULT => Fault,
			EIO => CanNotWriteBackPage,
			EINVAL => DirtyPageCanNotBeMoved,
			ENOENT => PageIsNotPresent,
			ENOMEM => OutOfMemoryOnTargetNode,

			unexpected @ _ => panic!("unexpected error code '{}'", unexpected),
		}
	}
}
