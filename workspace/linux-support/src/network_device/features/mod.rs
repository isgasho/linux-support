// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::c::bit_set::*;
use self::NETIF_F::*;


include!("FeatureGroup.rs");
include!("FeatureGroupChoice.rs");
include!("FeatureValue.rs");
include!("LinkModeBitSet.rs");


struct PrivateFlagBit(u32);

impl PrivateFlagBit
{
	fn find(name: ObjectName32, string_sets: &HashMap<ethtool_stringset, IndexSet<ObjectName32>>)
	{
		let string_set = string_sets.get(&ethtool_stringset::ETH_SS_PRIV_FLAGS).expect("Missing private flags string set");
	}
}