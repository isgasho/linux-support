// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn combine_multiple_application_services_with_protocols(service_field_enum_member_name: &'static str, subtype_field_name: &'static str, application_services: HashMap<ApplicationServiceTag, &'static str>, application_protocol_permutations: Vec<(Permutation<&'static str>, HashSetStaticName)>) -> HashMap<String, String>
{
	let mut result = HashMap::with_capacity(1024);
	
	for (application_service, subtype_identifier) in application_services
	{
		for (application_protocol_permutation, transport_protocols_hash_set_static_name) in application_protocol_permutations.iter()
		{
			let key = format!("{}{}", application_service, application_protocol_permutation_to_colon_delimited_string(application_protocol_permutation));
			
			if key.len() > MaximumServiceFieldSize
			{
				continue
			}
			
			let value = format!("{} {{ {}: {}, transport_protocols: &{} }}", service_field_enum_member_name, subtype_field_name, subtype_identifier, transport_protocols_hash_set_static_name);
			result.insert(key, value)
		}
	}
	
	result
}
