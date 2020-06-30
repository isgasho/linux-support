// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A register.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExtendedRegister
{
	/// `r0`.
	///
	/// Use for return codes and exit codes.
	///
	/// Also used for converted classic BPC (cBPF) programs for register `A`.
	///
	/// `BPF_REG_0`.
	r0 = 0,
	
	/// `r1`.
	///
	/// Upon program start contains the "context" argument pointer.
	///
	/// First argument passed to a function or eBPF program.
	///
	/// `BPF_REG_1` and `BPF_ARG_1`.
	r1 = 1,
	
	/// `r2`.
	///
	/// Second argument passed to a function or eBPF program.
	///
	/// `BPF_REG_2` and `BPF_ARG_2`.
	r2 = 2,
	
	/// `r3`.
	///
	/// Third argument passed to a function or eBPF program.
	///
	/// `BPF_REG_3` and `BPF_ARG_3`.
	r3 = 3,
	
	/// `r4`.
	///
	/// Fourth argument passed to a function or eBPF program.
	///
	/// `BPF_REG_4` and `BPF_ARG_4`.
	r4 = 4,
	
	/// `r5`.
	///
	/// Fifth argument passed to a function or eBPF program.
	///
	/// `BPF_REG_5` and `BPF_ARG_5`.
	r5 = 5,
	
	/// `r6`.
	///
	/// Preserved (callee-saved) across function calls.
	///
	/// Also used for a context (`BPF_REG_CTX`).
	///
	/// `BPF_REG_6`.
	r6 = 6,
	
	/// `r7`.
	///
	/// Preserved (callee-saved) across function calls.
	///
	/// Also used for converted classic BPC (cBPF) programs for register `X`.
	///
	/// `BPF_REG_7`.
	r7 = 7,
	
	/// `r8`.
	///
	/// Preserved (callee-saved) across function calls.
	///
	/// Also used for converted classic BPC (cBPF) programs for register `TMP`.
	///
	/// `BPF_REG_8`.
	r8 = 8,
	
	/// `r9`.
	///
	/// Preserved (callee-saved) across function calls.
	///
	/// `BPF_REG_9`.
	r9 = 9,
	
	/// Also known as `fp`.
	///
	/// Stores frame pointer to access the stack, which is 512 bytes in size.
	///
	/// Read-only.
	///
	/// `BPF_REG_10`.
	r10 = 10,
}

impl ExtendedRegister
{
	/// An alias to `r10`.
	pub const fp: Self = ExtendedRegister::r10;
}
