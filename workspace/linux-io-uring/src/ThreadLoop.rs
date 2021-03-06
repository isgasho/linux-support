// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sadly the use of `queues` and `dog_stats_d_publisher` make this a self-owning struct.
///
/// See <https://gist.github.com/tomaka/da8c374ce407e27d5dac> for some more information on why these are problematic in Rust.
#[derive(Debug)]
pub struct ThreadLoop<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize>
{
	io_uring: Rc<IoUring<'static>>,
	registered_buffers: RegisteredBuffers,
	signal_file_descriptor: SignalFileDescriptor,
	our_hyper_thread: HyperThread,
	coroutine_managers: ThreadLocalCoroutineManagers<CoroutineHeapSize, GTACSA, AcceptStackSize>,
	retry_submission_queue_was_full_coroutine_instance_handle: Option<CoroutineInstanceHandle>,
	dog_stats_d_publisher: DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
	subscriber: Subscriber<MessageHandlerArguments, DequeuedMessageProcessingError>,
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize> ThreadLoopBodyFunction for ThreadLoop<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	#[inline(always)]
	fn invoke<T: Terminate>(&mut self, terminate: &Arc<T>)
	{
		debug_assert!(terminate.should_continue());
		
		self.retry_coroutines_who_found_the_submission_queue_full();
		
		let exit = self.process_all_outstanding_completions(terminate);
		if unlikely!(exit)
		{
			return
		}

		let exit = self.process_thread_control_messages(terminate);
		if unlikely!(exit)
		{
			return
		}
		
		self.initiate_asynchronous_io()
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AcceptStackSize: MemorySize> ThreadLoop<CoroutineHeapSize, GTACSA, AcceptStackSize>
{
	#[inline(always)]
	fn retry_coroutines_who_found_the_submission_queue_full(&mut self)
	{
		let coroutine_instance_handle = match self.retry_submission_queue_was_full_coroutine_instance_handle.take()
		{
			None => return,
			Some(coroutine_instance_handle) => coroutine_instance_handle,
		};
		
		use self::CoroutineRequiresReEntry::*;
		match self.coroutine_managers.dispatch_retry_because_io_uring_submission_queue_was_full(coroutine_instance_handle)
		{
			CarryOn => (),
			
			IoUringSubmissionQueueWasFull(coroutine_instance_handle) =>
			{
				debug_assert!(self.retry_submission_queue_was_full_coroutine_instance_handle.is_none());
				self.retry_submission_queue_was_full_coroutine_instance_handle = Some(coroutine_instance_handle);
			}
		}
	}
	
	#[inline(always)]
	fn process_all_outstanding_completions(&mut self, terminate: &Arc<impl Terminate>) -> bool
	{
		let iterator = self.io_uring.current_completion_queue_entries();
		for completion_queue_entry in iterator
		{
			let user_data = completion_queue_entry.user_data();
			let completion_response = completion_queue_entry.completion_response();
			
			if CoroutineInstanceHandle::is_not_for_a_coroutine(user_data)
			{
				if let Err(cause) = self.non_coroutine_handler(user_data, completion_response)
				{
					terminate.begin_termination_due_to_irrecoverable_error(&cause, None);
					return true
				}
			}
			else
			{
				use self::CoroutineRequiresReEntry::*;
				match self.coroutine_managers.dispatch_io_uring(user_data, completion_response)
				{
					CarryOn => (),
					
					IoUringSubmissionQueueWasFull(coroutine_instance_handle) =>
					{
						debug_assert!(self.retry_submission_queue_was_full_coroutine_instance_handle.is_none());
						self.retry_submission_queue_was_full_coroutine_instance_handle = Some(coroutine_instance_handle);
						break
					}
				}
			}
		}
		false
	}
	
	#[inline(always)]
	fn process_thread_control_messages(&mut self, terminate: &Arc<impl Terminate>) -> bool
	{
		use self::DequeuedMessageProcessingError::*;
		
		static message_handler_arguments: MessageHandlerArguments = ();
		
		let result = self.subscriber.receive_and_handle_messages(terminate, &message_handler_arguments);
		match result
		{
			Ok(()) => false,
			
			Err(CarryOn(ref cause)) =>
			{
				self.log_carry_on(cause);
				false
			}
			
			Err(Fatal(cause)) =>
			{
				terminate.begin_termination_due_to_irrecoverable_error(&cause, None);
				true
			}
		}
	}
	
	#[inline(always)]
	fn initiate_asynchronous_io(&mut self)
	{
		match self.io_uring.initiate_asynchronous_io()
		{
			Ok(_number_of_submission_queue_entries_successfully_consumed) => (),
			
			Err(submit_error) => self.log_io_uring_submit_error(submit_error),
		}
	}
	
	#[inline(always)]
	fn non_coroutine_handler(&self, user_data: u64, completion_response: CompletionResponse) -> Result<(), DispatchIoUringError<io::Error>>
	{
		unimplemented!("We have 63-bits of user data available, eg as a tagged pointer {} {:?}", user_data, completion_response)
	}
	
	#[inline(always)]
	fn log_io_uring_submit_error(&mut self, cause: SubmitError)
	{
		self.log(alert!("IoUringSubmitError", "SubmitError", Normal, Error), format_args!("{}", cause))
	}
	
	#[inline(always)]
	fn log_carry_on(&mut self, cause: &Box<dyn error::Error>)
	{
		self.log(alert!("CarryOn", "CarryOn", Normal, Error), format_args!("{}", cause))
	}
	
	#[inline(always)]
	fn log(&mut self, alert: &'static EventTemplate, message: Arguments)
	{
		self.dog_stats_d_publisher.log(alert, AdditionalDogStatsDTags::new(), message)
	}
}
