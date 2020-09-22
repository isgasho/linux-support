// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Instance.
#[derive(Debug)]
pub struct ExpressDataPathInstance<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue>
{
	user_memory: ManuallyDrop<UserMemory<FFQ>>,
	
	redirect_map_and_attached_program: ManuallyDrop<RedirectMapAndAttachedProgram>,
	
	network_interface_index: NetworkInterfaceIndex,
	
	force_copy: bool,
	
	force_zero_copy: bool,

	marker: PhantomData<ROTOB>,
}

impl<RingQueueDepths: CreateReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, FFQ>
{
	/// Converts to single-threaded owned instance.
	pub fn owned(self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, arguments: RingQueueDepths::Arguments) -> Result<OwnedExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth>, ExpressDataPathSocketCreationError>
	{
		Ok
		(
			OwnedExpressDataPathSocket
			{
				common:
				{
					let user_memory_socket_file_descriptor = self.user_memory.user_memory_socket_file_descriptor();
					let express_data_path_socket_file_descriptor = user_memory_socket_file_descriptor;
				
					CommonExpressDataPathSocket::new(express_data_path_socket_file_descriptor, self.network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::empty(), self.force_copy, self.force_zero_copy, user_memory_socket_file_descriptor, queue_identifier, defaults, &self.redirect_map_and_attached_program, arguments)?
				},
				
				instance: ManuallyDrop::new(self),
			}
		)
	}
	
	/// Converts to instance that can then create per-thread instances (sic).
	///
	/// If using a shared instance, then each shared instance should:-
	///
	/// * Have a transmit queue depth equal to the completion queue depth;
	/// * Have a receive queue depth equal to the fill queue depth;
	///
	/// Additionally, the `number_of_chunks` should be equal to the total of all shared instances transmit queue depths + receive queue depths, unless only forwarding immediately, in which case it can be half this value.
	#[inline(always)]
	pub fn shareable(self, number_of_threads_guess: NonZeroUsize) -> ShareableExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, FFQ::CS>
	{
		ShareableExpressDataPathInstance(Arc::new((self, BestForCompilationTargetSpinLock::new(), BestForCompilationTargetSpinLock::new(), Mutex::new(HashSet::with_capacity(number_of_threads_guess.get())))))
	}
	
	#[inline(always)]
	fn shared(&self, receive_or_transmit_or_both_ring_queue_depths: RingQueueDepths, queue_identifier: QueueIdentifier, defaults: &DefaultPageSizeAndHugePageSizes, arguments: RingQueueDepths::Arguments, instance: &ShareableExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, FFQ::CS>, express_data_path_socket_file_descriptor: ExpressDataPathSocketFileDescriptor) -> Result<SharedExpressDataPathSocket<RingQueueDepths::ReceiveOrTransmitOrBoth>, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let redirect_map_and_attached_program = &self.redirect_map_and_attached_program;
		
		if redirect_map_and_attached_program.is_our_owned_program_and_thus_can_not_be_shared()
		{
			return Err(AttachedExpressDataPathProgramNotSuitableForSharing)
		}
		
		Ok
		(
			SharedExpressDataPathSocket
			{
				common:
				{
					let user_memory_socket_file_descriptor = self.user_memory.user_memory_socket_file_descriptor();
					
					CommonExpressDataPathSocket::new(&express_data_path_socket_file_descriptor, self.network_interface_index, receive_or_transmit_or_both_ring_queue_depths, XdpSocketAddressFlags::SharedUserMemory, self.force_copy, self.force_zero_copy, user_memory_socket_file_descriptor, queue_identifier, defaults, redirect_map_and_attached_program, arguments)?
				},
				
				instance: instance.clone(),
				
				express_data_path_socket_file_descriptor: ManuallyDrop::new(express_data_path_socket_file_descriptor),
				
				queue_identifier,
			}
		)
	}
}

impl<RingQueueDepths: FillOrCompletionOrBothRingQueueDepths, FFQ: FreeFrameQueue> ExpressDataPathInstance<RingQueueDepths::ReceiveOrTransmitOrBoth, FFQ>
{
	/// New instance.
	#[inline(always)]
	pub fn new(number_of_chunks: NonZeroU32, frame_headroom: FrameHeadroom, chunk_size: FFQ::CS, fill_or_completion_or_both_ring_queue_depths: RingQueueDepths, huge_memory_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes, redirect_map_and_attached_program: Either<RedirectMapAndAttachedProgramSettings, RedirectMapAndAttachedProgram>, network_interface_name: NetworkInterfaceName, force_copy: bool, force_zero_copy: bool) -> Result<Self, ExpressDataPathSocketCreationError>
	{
		let (network_interface_index, network_interface_maximum_transmission_unit_including_frame_check_sequence) = Self::network_interface_index_and_network_interface_maximum_transmission_unit_including_frame_check_sequence(network_interface_name, chunk_size, frame_headroom)?;
		
		let user_memory = ManuallyDrop::new(UserMemory::new(number_of_chunks, chunk_size, frame_headroom, network_interface_maximum_transmission_unit_including_frame_check_sequence, fill_or_completion_or_both_ring_queue_depths, huge_memory_page_size, defaults)?);
		
		Ok
		(
			Self
			{
				redirect_map_and_attached_program: ManuallyDrop::new
				(
					match redirect_map_and_attached_program
					{
						Left(settings) => RedirectMapAndAttachedProgram::new_suitable_for_owned_or_reuse_already_attached(network_interface_index, settings, queue_identifier)?,
						
						Right(redirect_map_and_attached_program) => redirect_map_and_attached_program,
					}
				),
				
				network_interface_index,
				
				force_copy,
				
				force_zero_copy,
				
				marker: PhantomData,
				
				user_memory,
			}
		)
	}
}

impl<ROTOB: ReceiveOrTransmitOrBoth, FFQ: FreeFrameQueue> ExpressDataPathInstance<ROTOB, FFQ>
{
	#[inline(always)]
	fn network_interface_index_and_network_interface_maximum_transmission_unit_including_frame_check_sequence(network_interface_name: NetworkInterfaceName, chunk_size: FFQ::CS, frame_headroom: FrameHeadroom) -> Result<(NetworkInterfaceIndex, MaximumTransmissionUnit), ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let network_device_control = NetworkDeviceInputOutputControl::new(Cow::Owned(network_interface_name)).map_err(CouldNotCreateNetworkDeviceControlSocket)?;
		
		Ok
		(
			(
				network_device_control.network_interface_name_to_network_interface_index().map_err(CouldNotGetValidNetworkInterfaceName)?.ok_or(NoSuchNetworkInterfaceName)?,
				Self::network_interface_maximum_transmission_unit_including_frame_check_sequence(chunk_size, frame_headroom, network_device_control)?
			)
		)
	}
	
	#[inline(always)]
	fn network_interface_maximum_transmission_unit_including_frame_check_sequence(chunk_size: AlignedChunkSize, frame_headroom: FrameHeadroom, network_device_control: NetworkDeviceInputOutputControl) -> Result<MaximumTransmissionUnit, ExpressDataPathSocketCreationError>
	{
		use self::ExpressDataPathSocketCreationError::*;
		
		let network_interface_maximum_transmission_unit_including_frame_check_sequence = Self::calculate_maximum_transmission_unit_including_frame_check_sequence(frame_headroom, chunk_size).map_err(|reason| CouldNotFindAnAcceptableMaximumTransmissionUnit { reason, frame_headroom, chunk_size })?;
		
		network_device_control.set_maximum_transmission_unit(network_interface_maximum_transmission_unit_including_frame_check_sequence)?.ok_or(NoSuchNetworkInterfaceName)?;
		
		Ok(network_interface_maximum_transmission_unit_including_frame_check_sequence)
	}
	
	/// Calculate the maximum transmission unit (MTU) that can be supported by eXpress Data Path.
	///
	/// Before creating an eXpress Data Path socket,
	#[inline(always)]
	fn calculate_maximum_transmission_unit_including_frame_check_sequence(frame_headroom: FrameHeadroom, chunk_size: AlignedChunkSize) -> Result<MaximumTransmissionUnit, String>
	{
		let chunk_size = chunk_size.into() as u32;
		debug_assert!((chunk_size as usize) >= (XDP_PACKET_HEADROOM + MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence.into()));
		
		let without_frame_headroom = chunk_size.checked_sub(frame_headroom.0).ok_or(format!("frame_headroom `{:?}` leaves no space whatsoever for ethernet frame", frame_headroom))?;
		match without_frame_headroom.checked_sub(MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence.0)
		{
			None => Err(format!("frame_headroom `{:?}` leaves no space for even the minimum ethernet frame size of `{:?}`", frame_headroom, MaximumTransmissionUnit::EthernetInclusiveMinimumIncludingFrameCheckSequence)),
			
			Some(value) => Ok(MaximumTransmissionUnit(value)),
		}
	}
}
