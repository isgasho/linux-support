// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::syscall::SYS;

/*

SO_BPF_EXTENSIONS
SO_ATTACH_BPF
SO_INCOMING_NAPI_ID
	- can be used by a BPF program to direct packets based on RX queue, I think.
Explore aRFS.
 */

include!("__BindgenBitfieldUnit.rs");
include!("__IncompleteArrayField.rs");
include!("_bindgen_ty_1.rs");
include!("_bindgen_ty_2.rs");
include!("AlignedU64.rs");
include!("bpf.rs");
include!("bpf_attach_type.rs");
include!("bpf_attr.rs");
include!("bpf_cgroup_storage_key.rs");
include!("bpf_cmd.rs");
include!("bpf_insn.rs");
include!("bpf_lpm_trie_key.rs");
include!("bpf_map_type.rs");
include!("BPF_OBJ_NAME_LEN.rs");
include!("BPF_PROG_ATTACH_flags.rs");
include!("BPF_PROG_LOAD_flags.rs");
include!("BPF_PROG_QUERY_flags.rs");
include!("bpf_prog_type.rs");
include!("BPF_PSEUDO_CALL.rs");
include!("BPF_PSEUDO_MAP_.rs");
include!("bpf_stack_build_id.rs");
include!("bpf_stack_build_id_status.rs");
include!("BpfCommandBtfLoad.rs");
include!("BpfCommandGetIdentifier.rs");
include!("BpfCommandGetIdentifierValueOfIdentifier.rs");
include!("BpfCommandLinkCreate.rs");
include!("BpfCommandLinkUpdate.rs");
include!("BpfCommandMapBatch.rs");
include!("BpfCommandMapChange.rs");
include!("BpfCommandMapChangeValueOrNextKey.rs");
include!("BpfCommandMapCreate.rs");
include!("BpfCommandObject.rs");
include!("BpfCommandObjectGetInformationByFileDescriptor.rs");
include!("BpfCommandProgramAttachOrDetach.rs");
include!("BpfCommandProgramLoad.rs");
include!("BpfCommandProgramQuery.rs");
include!("BpfCommandProgramTestRun.rs");
include!("BpfCommandRawTracePointOpen.rs");
include!("BpfCommandTaskFileDescriptorQuery.rs");
include!("OffsetOrInternetProtocol.rs");

