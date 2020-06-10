// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive buffer size.
///
/// Note that the `setsockopt(SO_RCVBUF)` call will allocate twice the size of this value.
/// `getsockopt(SO_RCVBUF)` will return the actual allocated sized, not the requested size (ie returns the doubled size).
/// This is taken into account by our code; just set this value to the amount of memory wanted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ReceiveBufferSizeInBytes(NonZeroU32);

impl TryFrom<NonZeroU32> for ReceiveBufferSizeInBytes
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if unlikely!(value < Self::UsualInclusiveMinimumForTcp.0)
		{
			Err(ParseNumberError::TooSmall)
		}
		else if unlikely!(value > Self::InclusiveMaximumMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl ParseNumber for ReceiveBufferSizeInBytes
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let raw_value = NonZeroU32::parse_number(bytes, radix, parse_byte)?;
		Self::try_from(raw_value)
	}
}

impl ReceiveBufferSizeInBytes
{
	/// Typical value.
	///
	/// From `/proc/sys/net/core/rmem_max`.
	pub const UsualGlobalMaximum: Self = Self(unsafe { NonZeroU32::new_unchecked(212_992) });
	
	/// Default is `Self::UsualGlobalMaximum`.
	///
	/// Value of `/proc/sys/net/core/rmem_max`.
	#[inline(always)]
	pub fn global_maximum(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_core_rmem_max_file_path(proc_path).read_value().unwrap())
	}
	
	/// Default is `Self::UsualGlobalMaximum`.
	///
	/// Set value of `/proc/sys/net/core/rmem_max` if it exists.
	#[inline(always)]
	pub fn set_global_maximum(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/rmem_max");
		
		let file_path = Self::sys_net_core_rmem_max_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Typical value.
	///
	/// From `/proc/sys/net/core/rmem_default`.
	pub const UsualGlobalDefault: Self = Self(unsafe { NonZeroU32::new_unchecked(212_992) });
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/core/rmem_default`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_core_rmem_default_file_path(proc_path).read_value().unwrap())
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/core/rmem_default` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/rmem_default");
		
		let file_path = Self::sys_net_core_rmem_default_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Inclusive minimum.
	///
	/// From `/proc/sys/net/ipv4/tcp_rmem`, first column (one-based).
	pub const UsualInclusiveMinimumForTcp: Self = Self(unsafe { NonZeroU32::new_unchecked(4_096) });
	
	/// Typical value.
	///
	/// From `/proc/sys/net/ipv4/tcp_rmem`, second column (one-based); this value can be adjusted.
	pub const UsualGlobalDefaultForTcp: Self = Self(unsafe { NonZeroU32::new_unchecked(131_072) });
	
	/// Inclusive maximum.
	///
	/// From `/proc/sys/net/ipv4/tcp_rmem`, third column (one-based).
	pub const UsualInclusiveMaximumForTcp: Self = Self(unsafe { NonZeroU32::new_unchecked(6_291_456) });
	
	/// Default values are `Self::UsualInclusiveMinimumForTcp`, `Self::UsualGlobalDefaultForTcp` and `Self::UsualInclusiveMaximumForTcp`.
	///
	/// Value of `/proc/sys/net/ipv4/tcp_rmem`.
	#[inline(always)]
	pub fn global_tcp_minimum_default_and_maximum(proc_path: &ProcPath) -> io::Result<(Self, Self, Self)>
	{
		let line = Self::sys_net_ipv4_tcp_rmem_file_path(proc_path).read_raw_without_line_feed()?;
		let mut fields = line.splitn(3, |byte| *byte == b'\t');
		
		#[inline(always)]
		fn parse_field<'a>(fields: &mut impl Iterator<Item=&'a [u8]>) -> io::Result<ReceiveBufferSizeInBytes>
		{
			ReceiveBufferSizeInBytes::from_bytes(fields.next().unwrap()).map_err(|cause| io::Error::new(ErrorKind::Other, cause))
		}
		
		let minimum = parse_field(&mut fields)?;
		let default = parse_field(&mut fields)?;
		let maximum = parse_field(&mut fields)?;
		
		Ok((minimum, default, maximum))
	}
	
	/// Default values are `Self::UsualInclusiveMinimumForTcp`, `Self::UsualGlobalDefaultForTcp` and `Self::UsualInclusiveMaximumForTcp`.
	///
	/// Set value of `/proc/sys/net/ipv4/tcp_rmem` if it exists.
	#[inline(always)]
	pub fn set_global_tcp_minimum_default_and_maximum(proc_path: &ProcPath, (minimum, default, maximum): (Self, Self, Self)) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_rmem");
		
		let file_path = Self::sys_net_ipv4_tcp_rmem_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(format!("{}\t{}\t{}", minimum.0, default.0, maximum.0))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Upper limit on maximum values.
	pub const InclusiveMaximumMaximum: Self = Self(unsafe { NonZeroU32::new_unchecked(i32::MAX as u32) });
	
	#[inline(always)]
	const fn adjust_for_tcp_set_sock_opt(self) -> u32
	{
		let receive_buffer_size_in_bytes = self.0.get();
		receive_buffer_size_in_bytes / 2
	}
	
	#[inline(always)]
	fn sys_net_core_rmem_max_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("rmem_max")
	}
	
	#[inline(always)]
	fn sys_net_core_rmem_default_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("rmem_default")
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_rmem_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_rmem")
	}
}
