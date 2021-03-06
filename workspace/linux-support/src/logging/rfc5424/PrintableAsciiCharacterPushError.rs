// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when pushing.
#[derive(Debug)]
pub enum PrintableAsciiCharacterPushError
{
	#[allow(missing_docs)]
	Capacity(CapacityError<PrintableAsciiCharacter>),
	
	#[allow(missing_docs)]
	NotAPrintableAsciiCharacter(u8),
	
	#[allow(missing_docs)]
	DeniedPrintableAsciiCharacter(u8),
}

impl Display for PrintableAsciiCharacterPushError
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for PrintableAsciiCharacterPushError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::PrintableAsciiCharacterPushError::*;
		
		match self
		{
			&Capacity(ref cause) => Some(cause),
			
			&NotAPrintableAsciiCharacter(..) => None,
			
			&DeniedPrintableAsciiCharacter(..) => None,
		}
	}
}

impl From<CapacityError<PrintableAsciiCharacter>> for PrintableAsciiCharacterPushError
{
	#[inline(always)]
	fn from(error: CapacityError<PrintableAsciiCharacter>) -> Self
	{
		PrintableAsciiCharacterPushError::Capacity(error)
	}
}
