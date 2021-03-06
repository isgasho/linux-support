// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Various ways of telling mark how to resolve a path.
#[derive(Debug)]
pub enum MarkPath<'a>
{
	/// Use the path of an open directory file descriptor.
	ByDirectoryFileDescriptor
	{
		/// Directory.
		///
		/// Use DirectoryFileDescriptor::AlwaysCurrentWorkingDirectory to use the current working directory.
		directory: &'a DirectoryFileDescriptor,
	},

	/// Use an absolute path.
	AbsolutePath
	{
		/// Absolute path.
		absolute_path: &'a CStr,
	},

	/// Use a path relative to the path of an open directory file descriptor
	RelativeToDirectoryFileDescriptor
	{
		/// Path relative to `directory`.
		relative_path: &'a CStr,

		/// Directory.
		///
		/// Use DirectoryFileDescriptor::AlwaysCurrentWorkingDirectory to be relative to the current working directory.
		directory: &'a DirectoryFileDescriptor,
	},
}

impl<'a> MarkPath<'a>
{
	#[inline(always)]
	pub(crate) fn to_dirfd_and_pathname(&'a self) -> (RawFd, *const c_char)
	{
		use self::MarkPath::*;

		match self
		{
			&ByDirectoryFileDescriptor { directory } => (directory.as_raw_fd(), null()),
			&AbsolutePath { absolute_path } => (-1, absolute_path.as_ptr()),
			&RelativeToDirectoryFileDescriptor { directory, relative_path } => (directory.as_raw_fd(), relative_path.as_ptr()),
		}
	}
}
