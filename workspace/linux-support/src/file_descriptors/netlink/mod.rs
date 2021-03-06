// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



use self::c::*;
use self::message_flags::*;
use self::request::*;
use self::route::*;
use super::*;
use super::socket::c::*;
use crate::memory::PageSize;
use crate::process::ProcessIdentifier;
use crate::file_descriptors::socket::{new_socket, bind_socket, SocketCreationOrBindError};


/// Attributes.
pub(crate) mod attributes;


/// libc support structures.
pub mod c;


/// Message flags.
pub mod message_flags;


/// Request.
pub mod request;


/// Route.
pub mod route;


include!("AcknowledgmentOnlyReplyReceiver.rs");
include!("DumpCompleted.rs");
include!("DumpWasInterrupted.rs");
include!("MultipartMessagePartIdentification.rs");
include!("NETLINK_ALIGN.rs");
include!("NetlinkProtocol.rs");
include!("NetlinkSocketFileDescriptor.rs");
include!("PortIdentifier.rs");
include!("ReplyReceiver.rs");
include!("SequenceNumber.rs");
