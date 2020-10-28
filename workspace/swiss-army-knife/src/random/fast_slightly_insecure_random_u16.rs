// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fast, but slightly insecure, random u16.
#[allow(deprecated)]
#[inline(always)]
pub fn fast_slightly_insecure_random_u16() -> u16
{
	let mut random_value= unsafe { uninitialized() };
	
	loop
	{
		match unsafe { _rdrand16_step(&mut random_value) }
		{
			0 => break,
			
			1 => continue,
			
			unexpected @ _ => unreachable!("Intel _rdrand16_step() intrisnice returned a result other than 0 or 1: {}", unexpected)
		};
	}
	
	random_value
}
