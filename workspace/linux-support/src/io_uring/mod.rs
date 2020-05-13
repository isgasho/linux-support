// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::IORING_OP::*;
use self::c::*;
use self::non_null::*;
use self::offsets::*;
use super::LoadNonAtomically;
use crate::cpu::HyperThread;
use crate::file_descriptors::*;
use crate::file_descriptors::directory::*;
use crate::file_descriptors::directory::c::*;
use crate::file_descriptors::epoll::*;
use crate::file_descriptors::epoll::c::*;
use crate::file_descriptors::eventfd::EventFileDescriptor;
use crate::file_descriptors::file::*;
use crate::file_descriptors::pipes_and_fifos::*;
use crate::file_descriptors::socket::*;
use crate::file_descriptors::socket::c::*;
use crate::file_descriptors::memfd::MemoryFileDescriptor;
use crate::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use crate::memory::mapping::*;
use crate::poll::*;
use crate::io_priority::CompressedIoPriority;
use bitflags::bitflags;
use errno::errno;
use libc::AT_EMPTY_PATH;
use libc::AT_NO_AUTOMOUNT;
use libc::AT_SYMLINK_NOFOLLOW;
use libc::c_char;
use libc::c_void;
use libc::EACCES;
use libc::EADDRINUSE;
use libc::EAFNOSUPPORT;
use libc::EAGAIN;
use libc::EALREADY;
use libc::EBADF;
use libc::ECONNABORTED;
use libc::ECONNREFUSED;
use libc::ECONNRESET;
use libc::EDESTADDRREQ;
use libc::EBUSY;
use libc::ECANCELED;
use libc::EDQUOT;
use libc::EEXIST;
use libc::EFAULT;
use libc::EFBIG;
use libc::EINPROGRESS;
use libc::EISCONN;
use libc::EISDIR;
use libc::EIO;
use libc::EINTR;
use libc::EINVAL;
use libc::ELOOP;
use libc::EMFILE;
use libc::EMSGSIZE;
use libc::ENAMETOOLONG;
use libc::ENETUNREACH;
use libc::ENFILE;
use libc::ENOBUFS;
use libc::ENODEV;
use libc::ENOENT;
use libc::ENOMEM;
use libc::ENOSPC;
use libc::ENOSR;
use libc::ENOSYS;
use libc::ENOTCONN;
use libc::ENOTDIR;
use libc::ENOTSOCK;
use libc::ENXIO;
use libc::EOPNOTSUPP;
use libc::EOVERFLOW;
use libc::EPERM;
use libc::EPIPE;
use libc::EPROTO;
use libc::EPROTONOSUPPORT;
use libc::EROFS;
use libc::ESOCKTNOSUPPORT;
use libc::ESPIPE;
use libc::ETIME;
use libc::ETIMEDOUT;
use libc::ETXTBSY;
use libc::mode_t;
use libc::O_CLOEXEC;
use libc::O_CREAT;
use libc::O_DIRECT;
use libc::O_DIRECTORY;
use libc::O_EXCL;
use libc::O_NOCTTY;
use libc::O_NOFOLLOW;
use libc::O_NONBLOCK;
use libc::O_PATH;
use libc::O_RDONLY;
use libc::O_RDWR;
use libc::O_TMPFILE;
use libc::O_WRONLY;
use libc::socklen_t;
use libc::sigset_t;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::{max, Ordering};
use std::collections::BTreeSet;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::mem::size_of;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::ops::Add;
use std::ops::RangeInclusive;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;
use std::sync::atomic::AtomicU32;
use std::marker::PhantomData;
use crate::file_descriptors::path::PathFileDescriptor;
use crate::file_descriptors::block_device::BlockDeviceFileDescriptor;
use crate::file_descriptors::character_device::CharacterDeviceFileDescriptor;
use crate::file_descriptors::terminal::TerminalFileDescriptor;
use std::hash::{Hash, Hasher};


mod c;


mod non_null;


/// Offsets.
pub mod offsets;


include!("BufferGroup.rs");
include!("CompletionResponse.rs");
include!("CompletionQueueEntry.rs");
include!("CompletionQueueRing.rs");
include!("CompletionQueueRingIterator.rs");
include!("FileDescriptorKind.rs");
include!("FileDescriptorOrigin.rs");
include!("FileSynchronize.rs");
include!("IORING_OP.rs");
include!("IoUring.rs");
include!("IoUringCreationError.rs");
include!("IoUringFileDescriptor.rs");
include!("LinuxKernelSubmissionQueuePollingThreadConfiguration.rs");
include!("OpenOnDisk.rs");
include!("PersonalityCredentialsIdentifier.rs");
include!("RegisteredBufferIndex.rs");
include!("RelativeOrAbsoluteTimeout.rs");
include!("SendOrReceiveFlags.rs");
include!("SubmissionQueueRing.rs");
include!("SubmissionQueueEntry.rs");
include!("SubmissionQueueEntryFlags.rs");
include!("SubmissionQueueEntryOptions.rs");
include!("SubmitError.rs");
include!("SupportedFileDescriptor.rs");
include!("UserData.rs");
