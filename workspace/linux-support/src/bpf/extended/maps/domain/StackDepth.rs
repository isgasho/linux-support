// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Stack depth.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct StackDepth(NonZeroU8);

impl TryFrom<u16> for StackDepth
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			let non_zero = unsafe { NonZeroU16::new_unchecked(value) };
			Self::try_from(non_zero)
		}
	}
}

impl TryFrom<NonZeroU32> for StackDepth
{
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if value > Self::InclusiveMaximum
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl StackDepth
{
	/// `sysctl_perf_event_max_stack` is a global static defined initially as `PERF_MAX_STACK_DEPTH`.
	const PERF_MAX_STACK_DEPTH: NonZeroU16 = Self::new_unsafe((1 << ValueSizeU32::PAGE_SHIFT) as u16);
	
	pub const InclusiveMaximum: Self = Self(Self::PERF_MAX_STACK_DEPTH);
	
	#[inline(always)]
	pub(crate) const fn to_non_zero_u32<T: Sized>(self) -> NonZeroU32
	{
		unsafe { NonZeroU32::new_unchecked((self.0.get() as u32) * (size_of::<T>() as u32)) }
	}
	
	#[inline(always)]
	const fn new_unsafe(value: u16) -> Self
	{
		Self(unsafe { NonZeroU16::new_unchecked(value) })
	}
}
