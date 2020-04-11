// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Original environment of a process when it was started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEnvironment(HashMap<Box<[u8]>, Box<[u8]>>);

impl Deref for OriginalEnvironment
{
	type Target = HashMap<Box<[u8]>, Box<[u8]>>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl FromBytes for OriginalEnvironment
{
	type Error = io::Error;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		match parse_ascii_nul_string_values::<HashMap<Box<[u8]>, Box<[u8]>>, _>(bytes, & |collection, ascii_string|
		{
			let mut iterator = ascii_string.splitn(2, |byte| *byte == b'=');
			let environment_variable_name = iterator.next().unwrap().to_vec().into_boxed_slice();
			let environment_variable_value = iterator.next().ok_or_else(|| "/proc/<pid>/environ value must have '='")?.to_vec().into_boxed_slice();
			if collection.insert(environment_variable_name, environment_variable_value).is_some()
			{
				return Err("/proc/<pid>/environ key duplicated")
			}
			Ok(())
		})
		{
			Ok(collection) => Ok(Self(collection)),
			Err(reason) => Err(io::Error::new(ErrorKind::Other, reason)),
		}
	}
}

impl OriginalEnvironment
{
	/// For self.
	#[inline(always)]
	pub fn for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::for_process(proc_path, ProcessIdentifierChoice::Current)
	}

	/// For process.
	#[inline(always)]
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let bytes = proc_path.process_file_path(process_identifier, "environ").read_raw()?;
		Self::from_bytes(&bytes)
	}

	/// Contains a specific environment variable?
	#[inline(always)]
	pub fn contains_environment_variable(&self, variable_name: &[u8]) -> bool
	{
		self.0.contains_key(variable_name)
	}

	/// Get a specific environment variable's value.
	#[inline(always)]
	pub fn get_environment_variable<'a>(&'a self, variable_name: &[u8]) -> Option<&'a Box<[u8]>>
	{
		self.0.get(variable_name)
	}

	/// Iterate.
	#[inline(always)]
	pub fn iterate(&self) -> impl Iterator<Item=(&Box<[u8]>, &Box<[u8]>)>
	{
		self.0.iter()
	}
}
