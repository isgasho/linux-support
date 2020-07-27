// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithNumaNodeSystemWideHashMapConstructor<K: Copy, V: Copy>(PhantomData<(K, V)>);

impl<K: Copy, V: Copy> MapConstructor for WithNumaNodeSystemWideHashMapConstructor<K, V>
{
	const Singleton: Self = Self(PhantomData);
	
	type Map = crate::bpf::extended::maps::HashMap<K, V>;
	
	type AccessPermissions = AccessPermissions;
	
	type InvariantArguments = Preallocation;
	
	type VariableArguments = NumaNode;
	
	#[inline(always)]
	fn construct(map_file_descriptors: &mut FileDescriptorsMap<MapFileDescriptor>, map_name: &MapName, parsed_bpf_type_format_map_data: Option<&ParsedBpfTypeFormatMapData>, maximum_entries: MaximumEntries, access_permissions: Self::AccessPermissions, invariant_arguments: Self::InvariantArguments, variable_arguments: Self::VariableArguments) -> Result<Self::Map, MapCreationError>
	{
		let preallocation = invariant_arguments;
		let numa_node = variable_arguments;
		crate::bpf::extended::maps::HashMap::new_system_wide(map_file_descriptors, map_name, parsed_bpf_type_format_map_data, maximum_entries, access_permissions, preallocation, Some(numa_node))
	}
}