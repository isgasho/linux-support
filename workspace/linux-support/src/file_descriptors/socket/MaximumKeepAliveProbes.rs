// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can not exceed `i8::MAX`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumKeepAliveProbes(u8);

impl TryFrom<u8> for MaximumKeepAliveProbes
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Default for MaximumKeepAliveProbes
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl MaximumKeepAliveProbes
{
	/// From `/proc/sys/net/ipv4/tcp_keepalive_probes`.
	pub const Default: Self = Self(9);
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(0);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(i8::MAX as u8);
	
	/// Value of `/proc/sys/net/ipv4/tcp_keepalive_probes`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_keepalive_probes_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_keepalive_probes` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/net/ipv4/tcp_keepalive_probes`");
		
		let file_path = Self::sys_net_ipv4_tcp_keepalive_probes_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_keepalive_probes_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_keepalive_probes")
	}
}
