// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::checks::*;
use crate::capabilities_and_privileges::*;
use crate::coredump::CoreDumpFilterFlags;
use crate::cpu::*;
use crate::environment::populate_clean_environment;
use crate::file_descriptors::close_all_open_file_descriptors_apart_from_standard;
use crate::file_descriptors::directory::AccessPermissions;
use crate::file_descriptors::epoll::set_maximum_number_of_watched_file_descriptors_per_user;
use crate::file_descriptors::file::leasing::*;
use crate::file_descriptors::inotify::*;
use crate::file_descriptors::pipes_and_fifos::*;
use crate::file_descriptors::posix_message_queues::*;
use crate::file_handles::NumberOfFileHandles;
use crate::io_priority::IoPriority;
use crate::io_priority::RealTimeOrBestEffortIoPriorityLevel;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))] use crate::ioports::*;
use crate::linux_kernel_asynchronous_io::set_maximum_number_of_kernel_asynchronous_io_events_per_user;
use crate::linux_kernel_command_line::*;
use crate::linux_kernel_modules::*;
use crate::linux_kernel_panic::*;
use crate::linux_kernel_version::*;
use crate::logging::*;
use crate::memory::*;
use crate::memory::huge_pages::*;
use crate::memory::mapping::LockAllMemory;
use crate::memory::system_v_shared_memory::message_queues::*;
use crate::paths::*;
use crate::personality::*;
use crate::process::*;
use crate::resource_limits::*;
use crate::seccomp::*;
use crate::scheduling::*;
use crate::swap::*;
use crate::signals::*;
use crate::thread::*;
use crate::time::c::tzset;
use crate::user_and_groups::*;
use crate::file_descriptors::netlink::NetlinkSocketFileDescriptor;
use crate::file_descriptors::netlink::route::RouteNetlinkProtocol;


/// Checks.
pub mod checks;


include!("GlobalConfiguration.rs");
include!("GlobalConfigurationError.rs");
include!("GlobalEPollConfiguration.rs");
include!("GlobalEPollConfigurationError.rs");
include!("GlobalFileDescriptorConfiguration.rs");
include!("GlobalFileDescriptorConfigurationError.rs");
include!("GlobalFileHandleConfiguration.rs");
include!("GlobalFileHandleConfigurationError.rs");
include!("GlobalFileLeasingConfiguration.rs");
include!("GlobalFileLeasingConfigurationError.rs");
include!("GlobalInotifyConfiguration.rs");
include!("GlobalInotifyConfigurationError.rs");
include!("GlobalKernelPanicConfiguration.rs");
include!("GlobalKernelPanicConfigurationError.rs");
include!("GlobalLinuxKernelAsynchronousIoConfiguration.rs");
include!("GlobalLinuxKernelAsynchronousIoConfigurationError.rs");
include!("GlobalLinuxKernelCommandLineConfiguration.rs");
include!("GlobalLinuxKernelCommandLineConfigurationError.rs");
include!("GlobalLinuxKernelSamePageMergingConfiguration.rs");
include!("GlobalLinuxKernelSamePageMergingConfigurationError.rs");
include!("GlobalLinuxModuleConfiguration.rs");
include!("GlobalLinuxModuleConfigurationError.rs");
include!("GlobalPipeConfiguration.rs");
include!("GlobalPipeConfigurationError.rs");
include!("GlobalPosixMessageQueueConfiguration.rs");
include!("GlobalPosixMessageQueueConfigurationError.rs");
include!("GlobalSchedulingConfiguration.rs");
include!("GlobalSchedulingConfigurationError.rs");
include!("GlobalSecurityConfiguration.rs");
include!("GlobalSecurityConfigurationError.rs");
include!("GlobalSwapConfiguration.rs");
include!("GlobalSwapConfigurationError.rs");
include!("GlobalSystemVMessageQueueConfiguration.rs");
include!("GlobalSystemVMessageQueueConfigurationError.rs");
include!("GlobalTransparentHugePagesConfiguration.rs");
include!("GlobalTransparentHugePagesConfigurationError.rs");
include!("LocaleName.rs");
include!("ProcessNiceConfiguration.rs");
include!("ProcessNiceConfigurationError.rs");
include!("ProcessConfiguration.rs");
include!("ProcessConfigurationError.rs");
include!("ProcessExecutor.rs");
include!("ProcessExecutorError.rs");
include!("ProcessIoPriorityConfiguration.rs");
include!("ProcessIoPriorityConfigurationError.rs");
include!("set_proc_sys_kernel_value.rs");
include!("set_value.rs");
include!("set_value_once.rs");
