// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, align(8))]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AlignedU64(pub u64);

impl<T: Sized> From<*const T> for AlignedU64
{
	#[inline(always)]
	fn from(value: *const T) -> Self
	{
		let value = value as usize;
		debug_assert_eq!(value % size_of::<u64>(), 0, "Not 8-byte aligned");
		Self(value as u64)
	}
}

impl<T: Sized> From<*mut T> for AlignedU64
{
	#[inline(always)]
	fn from(value: *mut T) -> Self
	{
		let value = value as usize;
		debug_assert_eq!(value % size_of::<u64>(), 0, "Not 8-byte aligned");
		Self(value as u64)
	}
}

impl<T: Sized> From<NonNull<T>> for AlignedU64
{
	#[inline(always)]
	fn from(value: NonNull<T>) -> Self
	{
		Self::from(value.as_ptr())
	}
}

impl<'a, T: Sized> From<&'a [T]> for AlignedU64
{
	#[inline(always)]
	fn from(value: &'a [T]) -> Self
	{
		Self::from(value.as_ptr())
	}
}

impl<'a, T: Sized> From<&'a mut [T]> for AlignedU64
{
	#[inline(always)]
	fn from(value: &'a mut [T]) -> Self
	{
		Self::from(value.as_mut_ptr())
	}
}

impl Display for AlignedU64
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&self.0, f)
	}
}

impl AlignedU64
{
	pub(crate) const Null: Self = Self(0);
}
