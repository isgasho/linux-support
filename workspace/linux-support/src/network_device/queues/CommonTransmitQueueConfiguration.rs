// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit queue common configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommonTransmitQueueConfiguration
{
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	#[serde(default)] pub maximum_rate_in_megabits_per_second: Option<Option<NonZeroU32>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	#[serde(default)] pub byte_limits_hold_time: Option<Milliseconds>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	///
	/// Not validated to be less than or equal to `self.current_byte_limit`.
	#[serde(default)] pub minimum_current_byte_limit: Option<Option<NonZeroU64>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	///
	/// Not validated to be equal to or greater than `self.current_byte_limit`.
	#[serde(default)] pub maximum_current_byte_limit: Option<NonZeroU64>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	///
	/// Not validated to be in the range `self.minimum_current_byte_limit ..= self.maximum_current_byte_limit`.
	#[serde(default)] pub current_byte_limit: Option<Option<NonZeroU64>>,
}

impl CommonTransmitQueueConfiguration
{
	/// Linux default.
	pub const fn linux_default() -> Self
	{
		Self
		{
			maximum_rate_in_megabits_per_second: None,
			
			byte_limits_hold_time: Some(Milliseconds(1000)),
			
			minimum_current_byte_limit: Some(None),
			
			maximum_current_byte_limit: Some(new_non_zero_u64(1879048192)),
			
			current_byte_limit: Some(None)
		}
	}
	
	#[inline(always)]
	fn configure(&self, sys_path: &SysPath, transmit_sysfs_queue: &TransmitSysfsQueue) -> Result<(), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		if let Some(maximum_rate_in_megabits_per_second) = self.maximum_rate_in_megabits_per_second
		{
			transmit_sysfs_queue.set_maximum_rate_in_megabits_per_second(sys_path, maximum_rate_in_megabits_per_second).map_err(CouldNotSetPerTransmitQueueMaximumRate)?;
		}
		
		if let Some(byte_limits_hold_time) = self.byte_limits_hold_time
		{
			transmit_sysfs_queue.set_byte_limits_hold_time(sys_path, byte_limits_hold_time).map_err(CouldNotSetPerTransmitQueueByteLimitsHoldTime)?
		}
		
		if let Some(minimum_current_byte_limit) = self.minimum_current_byte_limit
		{
			transmit_sysfs_queue.set_minimum_current_byte_limit(sys_path, minimum_current_byte_limit).map_err(CouldNotSetPerTransmitQueueMinimumCurrentByteLimit)?
		}
		
		if let Some(maximum_current_byte_limit) = self.maximum_current_byte_limit
		{
			transmit_sysfs_queue.set_maximum_current_byte_limit(sys_path, maximum_current_byte_limit).map_err(CouldNotSetPerTransmitQueueMaximumCurrentByteLimit)?
		}
		
		if let Some(current_byte_limit) = self.current_byte_limit
		{
			transmit_sysfs_queue.set_current_byte_limit(sys_path, current_byte_limit).map_err(CouldNotSetPerTransmitQueueCurrentByteLimit)?
		}
		
		Ok(())
	}
}
