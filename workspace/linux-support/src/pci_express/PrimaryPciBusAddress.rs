// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a primary PCI bus in a system.
///
/// A primary PCI bus:-
///
/// * Does not have a PCI device as a parent;
/// * Has a different file system layout in sysfs on Linux to a PCI bus within a device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
pub struct PrimaryPciBusAddress(PciBusAddress);

impl Deref for PrimaryPciBusAddress
{
	type Target = PciBusAddress;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl PrimaryPciBusAddress
{
	/// Details.
	#[inline(always)]
	pub fn bus(&self, sys_path: &SysPath) -> io::Result<PciBus>
	{
		self.0.bus(self.folder_path(sys_path))
	}
	
	/// Devices.
	#[inline(always)]
	pub fn devices(&self, sys_path: &SysPath) -> io::Result<impl Iterator<Item=PciDeviceAddress>>
	{
		let bus_address = self.0;
		let prefix =
		{
			let mut address: String = bus_address.into();
			address.push(':');
			address.into_bytes();
		};
		
		self.folder_path(sys_path).read_dir().map(|iterator| iterator.filter_map(move |dir_entry|
		{
			let dir_entry = match dir_entry
			{
				Err(_) => return None,
				Ok(dir_entry) => dir_entry,
			};
			
			let dir_entry = match dir_entry.file_type()
			{
				Err(_) => return None,
				
				Ok(file_type) => if likely!(file_type.is_dir())
				{
					dir_entry
				}
				else
				{
					return None
				}
			};
			
			let file_name = dir_entry.file_name();
			
			// `file_name` is of the format `XXXX:YY:AA.B` where `XXXX` is a hexadecimal domain, `YY` is a hexadecimal bus, `AA` is a hexadecimal devid and `B` is a hexadcimal function.
			const Template: &'static [u8] = b"XXXX:YY:AA.B";
			if file_name.len() != Template.len()
			{
				return None
			}
			
			if !file_name.starts_with(&prefix[..])
			{
				return None
			}
			
			let file_name_bytes = file_name.into_vec();
			
			let should_be_period = unsafe { * file_name_bytes.get_unchecked(11) };
			if unlikely!(should_be_period != b'.')
			{
				return None
			}
			
			let devid_hexadecimal = &file_name_bytes[8 .. 10];
			
			let function_hexadecimal = &file_name_bytes[12 .. 13];
			
			Some
			(
				PciDeviceAddress
				{
					bus_address,
					
					devid: match u8::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[8 .. 10])
					{
						Err(_) => return None,
						
						Ok(value) => if unlikely!(value > 31)
						{
							return None
						}
						else
						{
							value
						},
					},
					
					function: match u8::parse_hexadecimal_number_upper_or_lower_case(&file_name_bytes[12 .. 13])
					{
						Err(_) => return None,
						
						Ok(value) => if unlikely!(value > 15)
						{
							return None
						}
						else
						{
							value
						},
					},
				}
			)
		}))
	}
	
	#[inline(always)]
	fn folder_path(&self, sys_path: &SysPath) -> PathBuf
	{
		let address: String = self.into();
		sys_path.devices_folder_path().append(&format!("pci{}", address))
	}
	
	
}
