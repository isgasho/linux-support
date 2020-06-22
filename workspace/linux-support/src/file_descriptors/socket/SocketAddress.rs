// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can also be `SocketData` but not necessarily.
pub trait SocketAddress
{
	/// Socket data associated with this address.
	type SD: 'static + SocketData;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket server listener.
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, back_log: BackLog, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket client.
	///
	/// `writes_before_reading` is only appropriate for client sockets that send the first packet (ie write() before they read()).
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, writes_before_reading: bool, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket server listener.
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket client.
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>;
}
