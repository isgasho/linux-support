// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a process identifier.
///
/// Use cases for PID file descriptors:-
///
///  *  The pidfd_send_signal(2) system call can be used to send a signal to the process referred to by a PID file descriptor.
///  *  If the PID file descriptor refers to a child of the calling process, then it can be waited on using waitid(2).
///  *  A PID file descriptor can be monitored using poll(2), select(2), and epoll(7).
///
/// The latter is the most useful.
/// When the process that it refers to terminates, poll(2), select(2), and epoll(7) indicate the file descriptor as readable.
/// This can be used to monitor when child processes (created using clone) have terminated, rather than using a signal handler.
/// Note, however, that in the current implementation, nothing can be read from the file descriptor (read(2) on the file descriptor fails with the error EINVAL).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessIdentifierFileDescriptor(RawFd);

impl Drop for ProcessIdentifierFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for ProcessIdentifierFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ProcessIdentifierFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for ProcessIdentifierFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ProcessIdentifierFileDescriptor
{
}

impl ProcessIdentifierFileDescriptor
{
	/// Open.
	#[inline(always)]
	pub fn open(process_identifier: ProcessIdentifierChoice) -> Result<Self, CreationError>
	{
		let pid: pid_t = process_identifier.into();
		const flags: u32 = 0;
		let result = pidfd_open.syscall2(pid as usize, flags as usize);
		if likely!(result >= 0)
		{
			Ok(Self(result as RawFd))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;
			match errno().0
			{
				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),

				ENOMEM => Err(KernelWouldBeOutOfMemory),
				ESRCH => Err(ProcessForProcessIdentifierDoesNotExist),

				ENODEV => panic!("The anonymous inode filesystem is not available in this kernel"),
				EINVAL => panic!("flags is not 0 or pid is not valid"),

				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			panic!("Unexpected result from pidfd_open of `{}`", result)
		}
	}

	/// Send a signal.
	///
	/// The calling process must either be in the same PID namespace as the process referred to by this pidfd, or be in an ancestor of that namespace.
	///
	/// Since Linux 5.1.
	#[inline(always)]
	pub fn send_signal(&self, signal: Signal, signal_information: Option<&siginfo_t>) -> Result<(), SendSignalError>
	{
		let signo: i32 = signal.into();
		let information = match signal_information
		{
			Some(signal_information) => signal_information as *const siginfo_t,
			None => null(),
		};
		const flags: u32 = 0;
		let result = pidfd_send_signal.syscall4(self.0 as usize, signo as usize, information as usize, flags as usize);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::SendSignalError::*;

			match errno().0
			{
				EPERM => Err(PermissionDenied),

				ESRCH => Err(ProcessHasFinished),

				EINVAL => panic!("sig is not a valid signal, or, the calling process is not in a PID namespace from which it can send a signal to the target process, or, flags is not 0"),

				unexpected @ _ => panic!("Unexpected error number {}", unexpected),
			}
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}

	/// Permission to duplicate another process's file descriptor is governed by a ptrace access mode `PTRACE_MODE_ATTACH_REALCREDS` check.
	///
	/// See <http://man7.org/linux/man-pages/man2/pidfd_getfd.2.html>.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn duplicate_file_descriptor_from_other_process<FD: AsRawFd + FromRawFd>(&self, other_process_file_descriptor: &FD) -> Result<FD, CreationError>
	{
		const Flags: u32 = 0;
		let result = pidfd_getfd.syscall3(self.as_raw_fd() as usize, other_process_file_descriptor.as_raw_fd() as usize, Flags as usize) as i32;
		if likely!(result >= 0)
		{
			Ok(unsafe { FD::from_raw_fd(result) })
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			match errno().0
			{
				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
				EPERM => Err(PermissionDenied),
				ESRCH => Err(ProcessForProcessIdentifierDoesNotExist),

				EBADF => panic!("pidfd is not a valid PID file descriptor, or targetfd is not an open file descriptor in the process referred to by pidfd"),
				EINVAL => panic!("flags is not 0"),

				unexpected @ _ => panic!("Unexpected error {} from pidfd_getfd()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from pidfd_getfd()", result))
		}
	}
}
