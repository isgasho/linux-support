// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A program line that can be deserialized or serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ProgramLine<'name>
{
	/// Represents a label used for conditional and non-conditional jumps.
	Label(#[serde(borrow)] Name<'name>),
	
	/// Represents a function used with a relative function call.
	Function(#[serde(borrow)] Name<'name>, #[serde(skip)] Option<FunctionPrototype>),
	
	/// Represents a block of statements.
	#[serde(skip)] Block(Vec<Box<Self>>),
	
	/// Load a true 64-bit value.
	///
	/// `destination_register = immediate`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `immediate`.
	///
	/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
	LoadImmediate64(Register, #[serde(borrow)] Immediate<'name, u64>),
	
	/// Load a map file descriptor.
	///
	/// `destination_register = map_file_descriptor`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `map_file_descriptor`.
	///
	/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
	LoadMapFileDescriptor(Register, MapName),
	
	/// Load a map value.
	///
	/// `destination_register = map_file_descriptor`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `map_file_descriptor`.
	/// * `2`: `offset_into_value`.
	///
	/// ***CAUTION***: emits ***2*** instructions - be aware when manually counting jump offsets!
	LoadMapValue(Register, MapName, Immediate<'name, i32>),
	
	/// Operation on 32-bits of values.
	///
	/// `destination_register = destination_register operation source`.
	///
	/// # Fields
	///
	/// * `0`: `operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source`.
	Alu32(AluOperation, Register, #[serde(borrow)] RegisterOrImmediate<'name>),
	
	/// Operation on all 64-bits of values.
	///
	/// `destination_register = destination_register operation source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source`.
	Alu64(AluOperation, Register, #[serde(borrow)] RegisterOrImmediate<'name>),
	
	/// ?Uncertain of Encoding?
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `length` (eg `16` or `32`).
	ToLittleEndian(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// ?Uncertain of Encoding?
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `length` (eg `16` or `32`).
	ToBigEndian(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// Move of lower 32 bits.
	///
	/// `destination_register = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	Move32(Register, #[serde(borrow)] RegisterOrImmediate<'name>),
	
	/// Move of all 64 bits.
	///
	/// `destination_register = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	Move64(Register, #[serde(borrow)] RegisterOrImmediate<'name>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u8).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct8(#[serde(borrow)] Immediate<'name, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u16).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct16(#[serde(borrow)] Immediate<'name, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u32).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct32(#[serde(borrow)] Immediate<'name, i32>),
	
	/// Direct packet access.
	///
	/// Uses a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u64).add(immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `immediate`.
	LoadR0Direct64(#[serde(borrow)] Immediate<'name, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u8).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect8(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u16).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect16(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u32).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect32(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// Indirect packet access.
	///
	/// Uses a variable offset (`source_register`) with a constant offset (`immediate`).
	///
	/// `r0 = *((skb.data as *const u64).add(source_register + immediate))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `source_register`.
	/// * `1`: `immediate`.
	LoadR0Indirect64(Register, #[serde(borrow)] Immediate<'name, i32>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u8).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory8(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u16).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory16(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u32).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory32(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory load.
	///
	/// Uses a variable memory location (`source_register`) with a constant offset (`memory_offset`).
	///
	/// `destination_register = *((source_register as *const u64).add(memory_offset))`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`.
	/// * `2`: `memory_offset`.
	LoadFromMemory64(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u8).add(memory_offset)) = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	/// * `2`: `memory_offset`.
	StoreToMemory8(Register, RegisterOrImmediate<'name>, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u16).add(memory_offset)) = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	/// * `2`: `memory_offset`.
	StoreToMemory16(Register, RegisterOrImmediate<'name>, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u32).add(memory_offset)) = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	/// * `2`: `memory_offset`.
	StoreToMemory32(Register, RegisterOrImmediate<'name>, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store.
	///
	/// `*((destination_register as *mut u64).add(memory_offset)) = source`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source`.
	/// * `2`: `memory_offset`.
	StoreToMemory64(Register, RegisterOrImmediate<'name>, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u8).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd8(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u16).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd16(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u32).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd32(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Memory store using an atomic add.
	///
	/// `*((destination_register as *mut u64).add(memory_offset)) += source_register`.
	///
	///
	/// # Fields
	///
	/// * `0`: `destination_register`.
	/// * `1`: `source_register`
	/// * `2`: `memory_offset`.
	StoreToMemoryAtomicAdd64(Register, Register, #[serde(borrow)] MemoryOffset<'name>),
	
	/// Conditional jump after comparison of lower 32 bits.
	///
	/// ```bash
	/// if destination_register jump_operation source
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	///
	/// `program_counter` is also known as `pc`.
	///
	/// # Fields
	///
	/// * `0`: `jump_operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source`.
	/// * `3`: `program_counter_offset`.
	ConditionalJump32(JumpOperation, Register, #[serde(borrow)] RegisterOrImmediate<'name>, #[serde(borrow)] ProgramCounterOffset<'name, i16>),
	
	/// Conditional jump after comparison of all 64 bits.
	///
	/// ```bash
	/// if destination_register jump_operation source
	/// {
	/// 	goto program_counter + program_counter_offset
	/// }
	/// ```
	///
	/// `program_counter` is also known as `pc`.
	///
	///
	/// # Fields
	///
	/// * `0`: `jump_operation`.
	/// * `1`: `destination_register`.
	/// * `2`: `source`.
	/// * `3`: `program_counter_offset`.
	ConditionalJump64(JumpOperation, Register, #[serde(borrow)] RegisterOrImmediate<'name>, #[serde(borrow)] ProgramCounterOffset<'name, i16>),
	
	/// Unconditional jump.
	///
	/// `goto program_counter + program_counter_offset`.
	///
	/// `program_counter` is also known as `pc`.
	///
	///
	/// # Fields
	///
	/// * `0`: `program_counter_offset`.
	UnconditionalJump(#[serde(borrow)] ProgramCounterOffset<'name, i16>),
	
	/// Function call.
	///
	/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
	/// The function result will be returned in `r0`.
	/// The function `bpf_tail_call()` never returns if successfully invoked.
	///
	/// `call function_identifier`.
	///
	///
	/// # Fields
	///
	/// * `0`: `function_identifier`.
	FunctionCall(bpf_func_id),
	
	/// Relative function call.
	///
	/// Calls a BPF function within the same block of instructions.
	///
	/// Registers `r1` through to `r5` inclusive are used to pass function arguments and are clobbered.
	/// The function result will be returned in `r0`.
	///
	///
	/// # Fields
	///
	/// * `0`: `program_counter_offset`.
	RelativeFunctionCall(#[serde(borrow)] ProgramCounterOffset<'name, i32>),
	
	/// Program exit.
	///
	/// Returns the value in the register `r0`.
	ProgramExit,
}

impl<'name> ProgramLine<'name>
{
	/// Add to instruction(s).
	#[inline(always)]
	pub(crate) fn parse(&self, instructions: &mut ProgramLinesParser<'name>, i32_immediates_map: &OffsetsMap<i32>, u64_immediates_map: &OffsetsMap<u64>, memory_offsets_map: &OffsetsMap<i16>, map_file_descriptors: &FileDescriptorsMap<MapFileDescriptor>) -> Result<(), ParseError>
	{
		use self::ProgramLine::*;
		use self::RegisterOrImmediate::*;
		
		let instruction = match self
		{
			&Label(ref label) => return instructions.register_label(label),
			
			&Function(ref relative_function_name, ref function_prototype) =>
			{
				instructions.register_relative_function_name(relative_function_name, function_prototype.as_ref())?;
				return Ok(())
			}
			
			&Block(ref program_lines) =>
			{
				for program_line in program_lines
				{
					program_line.parse(instructions, i32_immediates_map, u64_immediates_map, memory_offsets_map, map_file_descriptors)?;
				}
				
				return Ok(())
			}
			
			&LoadImmediate64(destination_register, ref immediate) => return instructions.two_instructions(bpf_insn::load64_immediate(destination_register, u64_immediates_map.resolve(immediate)?)),
			
			&LoadMapFileDescriptor(destination_register, ref map_name) => return instructions.two_instructions(bpf_insn::load_map_file_descriptor(destination_register, map_file_descriptors.resolve(map_name)?)),
			
			&LoadMapValue(destination_register, ref map_name, ref offset_into_value) => return instructions.two_instructions(bpf_insn::load_map_value(destination_register, map_file_descriptors.resolve(map_name)?, i32_immediates_map.resolve(offset_into_value)?)),
			
			&Alu32(operation, destination_register, Register(source_register)) => bpf_insn::alu32(operation, destination_register, source_register),
			
			&Alu32(operation, destination_register, Immediate(ref immediate)) => bpf_insn::alu32_immediate(operation, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Alu64(operation, destination_register, Register(source_register)) => bpf_insn::alu64(operation, destination_register, source_register),
			
			&Alu64(operation, destination_register, Immediate(ref immediate)) => bpf_insn::alu64_immediate(operation, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&ToLittleEndian(destination_register, ref immediate) => bpf_insn::endian(EndiannessOperation::ToLittleEndian, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&ToBigEndian(destination_register, ref immediate) => bpf_insn::endian(EndiannessOperation::ToBigEndian, destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Move32(destination_register, Register(source_register)) => bpf_insn::move32(destination_register, source_register),
			
			&Move32(destination_register, Immediate(ref immediate)) => bpf_insn::move32_immediate(destination_register, i32_immediates_map.resolve(immediate)?),
			
			&Move64(destination_register, Register(source_register)) => bpf_insn::move64(destination_register, source_register),
			
			&Move64(destination_register, Immediate(ref immediate)) => bpf_insn::move64_immediate(destination_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct8(ref immediate) => bpf_insn::load8_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct16(ref immediate) => bpf_insn::load16_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct32(ref immediate) => bpf_insn::load32_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Direct64(ref immediate) => bpf_insn::load64_r0_direct(i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect8(source_register, ref immediate) => bpf_insn::load8_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect16(source_register, ref immediate) => bpf_insn::load16_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect32(source_register, ref immediate) => bpf_insn::load32_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadR0Indirect64(source_register, ref immediate) => bpf_insn::load64_r0_indirect(source_register, i32_immediates_map.resolve(immediate)?),
			
			&LoadFromMemory8(destination_register, source_register, ref memory_offset) => bpf_insn::load8_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory16(destination_register, source_register, ref memory_offset) => bpf_insn::load16_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory32(destination_register, source_register, ref memory_offset) => bpf_insn::load32_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&LoadFromMemory64(destination_register, source_register, ref memory_offset) => bpf_insn::load64_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory8(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store8_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory8(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store8_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory16(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store16_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory16(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store16_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory32(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store32_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory32(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store32_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemory64(destination_register, Register(source_register), ref memory_offset) => bpf_insn::store64_memory(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemory64(destination_register, Immediate(ref immediate), ref memory_offset) => bpf_insn::store64_memory_immediate(destination_register, memory_offsets_map.resolve(memory_offset)?, i32_immediates_map.resolve(immediate)?),
			
			&StoreToMemoryAtomicAdd8(destination_register, source_register, ref memory_offset) => bpf_insn::store8_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd16(destination_register, source_register, ref memory_offset) => bpf_insn::store16_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd32(destination_register, source_register, ref memory_offset) => bpf_insn::store32_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&StoreToMemoryAtomicAdd64(destination_register, source_register, ref memory_offset) => bpf_insn::store64_memory_atomic_add(destination_register, source_register, memory_offsets_map.resolve(memory_offset)?),
			
			&ConditionalJump32(jump_operation, destination_register, Register(source_register), ref program_counter_offset) => bpf_insn::conditional_jump32(jump_operation, destination_register, source_register, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump32(jump_operation, destination_register, Immediate(ref immediate), ref program_counter_offset) => bpf_insn::conditional_jump32_immediate(jump_operation, destination_register, i32_immediates_map.resolve(immediate)?, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump64(jump_operation, destination_register, Register(source_register), ref program_counter_offset) => bpf_insn::conditional_jump64(jump_operation, destination_register, source_register, instructions.resolve_label(program_counter_offset)?),
			
			&ConditionalJump64(jump_operation, destination_register, Immediate(ref immediate), ref program_counter_offset) => bpf_insn::conditional_jump64_immediate(jump_operation, destination_register, i32_immediates_map.resolve(immediate)?, instructions.resolve_label(program_counter_offset)?),
			
			&UnconditionalJump(ref program_counter_offset) => bpf_insn::unconditional_jump(instructions.resolve_label(program_counter_offset)?),
			
			&FunctionCall(function_identifier) => bpf_insn::function_call(function_identifier),
			
			&RelativeFunctionCall(ref program_counter_offset) => bpf_insn::relative_function_call(instructions.resolve_relative_function_name(program_counter_offset)?),
			
			&ProgramExit => bpf_insn::program_exit(),
		};
		
		instructions.one_instruction(instruction)
	}
}
