// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Transparent Huge Page (THP) shared memory ('shmem') choice.
///
/// Used for at least:-
///
/// * SysV SHM
/// * memfds,
/// * shared anonymous mmaps (of /dev/zero or `MAP_ANONYMOUS`)
/// * GPU drivers' DRM objects
/// * Ashmem
#[derive(Deserialize, Serialize)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransparentHugePageSharedMemoryChoice
{
	/// Never allocate.
	Never,

	/// Always use.
	Always,

	/// Only allocate huge page if it will be fully within 'i_size'.
	///
	/// Also for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	WithinSize,

	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	Advise,

	/// For use in emergencies, to force the huge option off from all mounts.
	Deny,

	/// Force the huge option on for all (very useful for testing).
	Force,
}

impl Default for TransparentHugePageSharedMemoryChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		TransparentHugePageSharedMemoryChoice::Never
	}
}

impl TransparentHugePageSharedMemoryChoice
{
	/// To value.
	#[inline(always)]
	pub fn to_value(self) -> &'static str
	{
		use self::TransparentHugePageSharedMemoryChoice::*;

		match self
		{
			Never => "never",
			Always => "always",
			WithinSize => "within_size",
			Advise => "advise",
			Deny => "deny",
			Force => "force",
		}
	}

	/// Changes Transparent Huge Pages (THP) settings.
	#[inline(always)]
	pub(crate) fn change_transparent_huge_pages_usage(self, sys_path: &SysPath) -> io::Result<()>
	{
		sys_path.global_transparent_huge_memory_file_path("shmem_enabled").write_value(self.to_value())
	}
}
