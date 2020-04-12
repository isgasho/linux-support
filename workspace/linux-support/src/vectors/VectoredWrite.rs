// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Implementors support vectored writes.
pub trait VectoredWrite
{
	/// Performs a vectored write.
	///
	/// All buffers should be non-zero sized otherwise the results are ambiguous (ie was nothing written or is this end-of-file).
	///
	/// For processes, see `ProcessIdentifier::write_vectored()`.
	fn write_vectored(&self, buffers: &[&[u8]]) -> io::Result<usize>;
}
