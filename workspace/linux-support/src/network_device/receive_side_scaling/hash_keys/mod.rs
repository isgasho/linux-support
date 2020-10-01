// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::network_device::c::flow_specifications::*;


include!("EthernetReceiveSideScalingFlowHashKey.rs");
include!("Layer4ReceiveSideScalingFlowHashKey.rs");
include!("InternetProtocolReceiveSideScalingFlowHashKey.rs");
include!("IpsecReceiveSideScalingFlowHashKey.rs");
include!("ReceiveSideScalingFlowHashKey.rs");
include!("ReceiveSideScalingFlowHashKeyConfiguration.rs");
include!("ReceiveSideScalingFlowHashKeyName.rs");
include!("RXH.rs");
include!("StreamTransmissionControlProtocolReceiveSideScalingFlowHashKey.rs");
include!("ToDataField.rs");
include!("TransmissionControlProtocolReceiveSideScalingFlowHashKey.rs");
include!("UserDatagramProtocolReceiveSideScalingFlowHashKey.rs");
