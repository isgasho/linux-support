// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `RRSIG` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RRSIGHandleRecordTypeError
{
	/// Resource data for resource record type `RRSIG` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Resource data for resource record type `RRSIG` has more than 126 labels (including root, only 127 labels are allowed and root is not allowed to be counted in this instance).
	HasMoreThan126Labels(u8),
	
	/// Security algorithm error.
	SecurityAlgorithmFailed(SecurityAlgorithmHandleRecordTypeError),
	
	/// Signers name.
	SignersName(ParsedNameParserError)
}

impl Display for RRSIGHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for RRSIGHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::RRSIGHandleRecordTypeError::*;
		
		match self
		{
			&SecurityAlgorithmFailed(ref error) => Some(error),
			
			&SignersName(ref error) => Some(error),
			
			_ => None,
		}
	}
}
