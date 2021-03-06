// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 7286.
fn application_layer_traffic_optimization() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"ALTO:http" => "ApplicationLayerTrafficOptimization { transport_protocol: Some(http), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_ApplicationLayerTrafficOptimization(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(http))? }",
		
		"ALTO:https" => "ApplicationLayerTrafficOptimization { transport_protocol: Some(https), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_ApplicationLayerTrafficOptimization(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(https))? }",
		
		// This is probably not intended by the authors of RFC 7286 but is permitted by RFC 3958.
		"ALTO" =>  "ApplicationLayerTrafficOptimization { transport_protocol: None, uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_ApplicationLayerTrafficOptimization(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, None)? }",
	}
}
