// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn allocate_child_process_stack(child_stack_allocator: &mut impl AllocRef, child_process_stack_size: usize) -> Result<(Layout, NonNull<u8>, NonNull<u8>), AllocError>
{
	debug_assert_eq!(child_process_stack_size % ChildStackAlignment, 0, "child_process_stack_size `{}` is not a multiple of 16", child_process_stack_size);

	let layout = unsafe { Layout::from_size_align_unchecked(child_process_stack_size, ChildStackAlignment) };
	let (bottom_of_child_stack_pointer, _actual_size_allocated) = child_stack_allocator.alloc(layout)?;
	let top_of_child_stack_pointer = top_of_child_stack_pointer(bottom_of_child_stack_pointer, child_process_stack_size);
	Ok((layout, bottom_of_child_stack_pointer, top_of_child_stack_pointer))
}
