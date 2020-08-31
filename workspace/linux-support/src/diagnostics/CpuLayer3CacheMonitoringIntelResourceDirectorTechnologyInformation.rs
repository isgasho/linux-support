// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuLayer3CacheMonitoringIntelResourceDirectorTechnologyInformation
{
	pub cache_conversion_factor: u32,
	
	pub cache_maximum_rmid_range: u32,
	
	pub cache_has_occupancy_monitoring: bool,
	
	pub cache_has_total_bandwidth_monitoring: bool,
	
	pub cache_has_local_bandwidth_monitoring: bool,
}
