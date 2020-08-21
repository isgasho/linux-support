// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct UsersAndGroupsDiagnostics
{
	pub process_identifier_choice: ProcessIdentifierChoice,
	
	pub users: UsersDiagnostics,
	
	pub groups: GroupsDiagnostics,
}

impl UsersAndGroupsDiagnostics
{
	fn gather(proc_path: &ProcPath, etc_path: &EtcPath, process_identifier_choice: ProcessIdentifierChoice) -> Self
	{
		Self
		{
			process_identifier_choice,
			users: UsersDiagnostics::gather(proc_path, etc_path, process_identifier_choice),
			groups: GroupsDiagnostics::gather(proc_path, etc_path, process_identifier_choice),
		}
	}
}
