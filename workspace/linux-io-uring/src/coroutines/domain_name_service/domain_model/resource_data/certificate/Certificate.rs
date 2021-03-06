// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A certificate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Certificate<OOPB: OwnedOrParsedBytes>
{
	/// Key tag.
	pub key_tag: KeyTag,

	/// Certificate algorithm.
	pub security_algorithm: SecurityAlgorithm,

	/// Certificate type and data.
	pub certificate_type: CertificateType<OOPB>,
}

impl<'message> Into<Certificate<OwnedBytes>> for Certificate<ParsedBytes<'message>>
{
	#[inline(always)]
	fn into(self) -> Certificate<OwnedBytes>
	{
		Certificate
		{
			key_tag: self.key_tag,
			security_algorithm: self.security_algorithm,
			certificate_type: self.certificate_type.into(),
		}
	}
}
