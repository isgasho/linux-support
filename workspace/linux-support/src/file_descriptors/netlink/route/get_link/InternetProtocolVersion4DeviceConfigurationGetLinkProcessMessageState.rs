// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct InternetProtocolVersion4DeviceConfigurationGetLinkProcessMessageState
{
	pub(crate) forwarding: Option<u32>,
	
	pub(crate) multicast_forwarding: Option<bool>,

	pub(crate) proxy_arp: Option<bool>,

	pub(crate) accept_redirects: Option<bool>,

	pub(crate) secure_redirects: Option<bool>,

	pub(crate) send_redirects: Option<bool>,

	pub(crate) shared_media: Option<u32>,

	pub(crate) reverse_path_filter: Option<u32>,

	pub(crate) accept_source_route: Option<bool>,

	pub(crate) boot_protocol_relay: Option<bool>,

	pub(crate) log_martians: Option<bool>,

	pub(crate) tag: Option<u32>,

	pub(crate) address_resolution_protocol_filter: Option<bool>,

	pub(crate) medium_identifier: Option<i32>,

	pub(crate) disable_xfrm: Option<bool>,

	pub(crate) disable_policy: Option<bool>,

	pub(crate) force_internet_group_management_protocol_version: Option<u32>,

	pub(crate) address_resolution_protocol_announce: Option<u32>,

	pub(crate) address_resolution_protocol_ignore: Option<u32>,

	pub(crate) promote_secondaries: Option<bool>,

	pub(crate) address_resolution_protocol_accept: Option<bool>,

	pub(crate) address_resolution_protocol_notify: Option<bool>,

	pub(crate) accept_local: Option<bool>,

	pub(crate) source_valid_mark: Option<u32>,

	pub(crate) proxy_address_resolution_protocol_pvlan: Option<bool>,

	pub(crate) route_localnet: Option<bool>,

	pub(crate) internet_group_management_protocol_version_2_unsolicited_report_interval: Option<Milliseconds>,

	pub(crate) internet_group_management_protocol_version_3_unsolicited_report_interval: Option<Milliseconds>,

	pub(crate) ignore_routes_with_link_down: Option<bool>,

	pub(crate) drop_unicast_in_layer2_multicast: Option<bool>,

	pub(crate) drop_gratuitous_address_resolution_protocol: Option<bool>,

	pub(crate) broadcast_forwarding: Option<bool>,
}

impl InternetProtocolVersion4DeviceConfigurationGetLinkProcessMessageState
{
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<InternetProtocolVersion4DeviceConfiguration, String>
	{
		Ok
		(
			InternetProtocolVersion4DeviceConfiguration
			{
				forwarding: self.forwarding.ok_or(format!("Linux kernel bug - missing forwarding"))?,
				multicast_forwarding: self.multicast_forwarding.ok_or(format!("Linux kernel bug - missing multicast_forwarding"))?,
				proxy_arp: self.proxy_arp.ok_or(format!("Linux kernel bug - missing proxy_arp"))?,
				accept_redirects: self.accept_redirects.ok_or(format!("Linux kernel bug - missing accept_redirects"))?,
				secure_redirects: self.secure_redirects.ok_or(format!("Linux kernel bug - missing secure_redirects"))?,
				send_redirects: self.send_redirects.ok_or(format!("Linux kernel bug - missing send_redirects"))?,
				shared_media: self.shared_media.ok_or(format!("Linux kernel bug - missing shared_media"))?,
				reverse_path_filter: InternetProtocolVersion4ReversePathFilter::parse(self.reverse_path_filter.ok_or(format!("Linux kernel bug - missing reverse_path_filter"))?)?,
				accept_source_route: self.accept_source_route.ok_or(format!("Linux kernel bug - missing accept_source_route"))?,
				boot_protocol_relay: self.boot_protocol_relay.ok_or(format!("Linux kernel bug - missing bootp_relay"))?,
				log_martians: self.log_martians.ok_or(format!("Linux kernel bug - missing log_martians"))?,
				tag: self.tag.ok_or(format!("Linux kernel bug - missing tag"))?,
				address_resolution_protocol_filter: self.address_resolution_protocol_filter.ok_or(format!("Linux kernel bug - missing arp_filter"))?,
				medium_identifier: InternetProtocolVersion4MediumIdentifier::parse(self.medium_identifier.ok_or(format!("Linux kernel bug - missing medium_id"))?),
				disable_xfrm: self.disable_xfrm.ok_or(format!("Linux kernel bug - missing disable_xfrm"))?,
				disable_policy: self.disable_policy.ok_or(format!("Linux kernel bug - missing disable_policy"))?,
				force_internet_group_management_protocol_version: InternetProtocolVersion4ForceInternetGroupManagementProtocolVersion::parse(self.force_internet_group_management_protocol_version.ok_or(format!("Linux kernel bug - missing force_internet_group_management_protocol_version"))?)?,
				address_resolution_protocol_announce: InternetProtocolVersion4AddressResolutionProtocolAnnounce::parse(self.address_resolution_protocol_announce.ok_or(format!("Linux kernel bug - missing arp_announce"))?)?,
				address_resolution_protocol_ignore: InternetProtocolVersion4AddressResolutionProtocolIgnore::parse(self.address_resolution_protocol_ignore.ok_or(format!("Linux kernel bug - missing arp_ignore"))?)?,
				promote_secondaries: self.promote_secondaries.ok_or(format!("Linux kernel bug - missing promote_secondaries"))?,
				address_resolution_protocol_accept: self.address_resolution_protocol_accept.ok_or(format!("Linux kernel bug - missing arp_accept"))?,
				address_resolution_protocol_notify: self.address_resolution_protocol_notify.ok_or(format!("Linux kernel bug - missing arp_notify"))?,
				accept_local: self.accept_local.ok_or(format!("Linux kernel bug - missing accept_local"))?,
				source_valid_mark: self.source_valid_mark.ok_or(format!("Linux kernel bug - missing source_valid_mark"))?,
				proxy_address_resolution_protocol_pvlan: self.proxy_address_resolution_protocol_pvlan.ok_or(format!("Linux kernel bug - missing proxy_arp_pvlan"))?,
				route_localnet: self.route_localnet.ok_or(format!("Linux kernel bug - missing route_localnet"))?,
				internet_group_management_protocol_version_2_unsolicited_report_interval: self.internet_group_management_protocol_version_2_unsolicited_report_interval.ok_or(format!("Linux kernel bug - missing internet_group_management_protocol_version_2_unsolicited_report_interval"))?,
				internet_group_management_protocol_version_3_unsolicited_report_interval: self.internet_group_management_protocol_version_3_unsolicited_report_interval.ok_or(format!("Linux kernel bug - missing internet_group_management_protocol_version_3_unsolicited_report_interval"))?,
				ignore_routes_with_link_down: self.ignore_routes_with_link_down.ok_or(format!("Linux kernel bug - missing ignore_routes_with_link_down"))?,
				drop_unicast_in_layer2_multicast: self.drop_unicast_in_layer2_multicast.ok_or(format!("Linux kernel bug - missing drop_unicast_in_layer2_multicast"))?,
				drop_gratuitous_address_resolution_protocol: self.drop_gratuitous_address_resolution_protocol.ok_or(format!("Linux kernel bug - missing drop_gratuitous_arp"))?,
				broadcast_forwarding: self.broadcast_forwarding.ok_or(format!("Linux kernel bug - missing broadcast_forwarding"))?,
			}
		)
	}
}
