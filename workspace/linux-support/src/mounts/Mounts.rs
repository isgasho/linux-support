// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mounts (or mount points) for the current process.
#[derive(Default, Debug)]
pub struct Mounts<'a>(HashMap<PathBuf, Mount<'a>>);

impl<'a> Mounts<'a>
{
	/// Current mounts (from `/proc/<X>/mounts`).
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let mounts_wrapper = MountsWrapper::new(&proc_path.process_file_path(process_identifier, "mounts"), true)?;

		let mut map = HashMap::with_capacity(64);

		mounts_wrapper.use_mount(|mount_point|
		{
			let key = mount_point.mount_point.clone();
			if let Some(previous) = map.insert(key, mount_point)
			{
				Err(io::Error::new(ErrorKind::InvalidData, format!("Duplicate mount_point for mount_point point '{:?}'", previous.mount_point)))
			}
			else
			{
				Ok(())
			}
		})?;

		Ok(Mounts(map))
	}

	/// Returns the first path for an existing `file_system_type` mount, if any.
	///
	/// Useful for specialized file systems like `cgroup2` and `hugetlbfs`.
	#[inline(always)]
	pub fn existing_mount(&self, file_system_type: FileSystemType) -> Option<&Path>
	{
		for mount in self.0.values()
		{
			if mount.has_file_system_type(&file_system_type)
			{
				let mount_point = &mount.mount_point;
				if mount_point.is_dir()
				{
					return Some(mount_point.as_path());
				}
			}
		}
		None
	}
}
