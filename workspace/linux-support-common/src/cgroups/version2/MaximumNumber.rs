/// Maximum number.
///
/// Defaults to `Maximum`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaximumNumber
{
	/// A finite value.
	///
	/// Can be zero.
	Finite(usize),

	/// A system defined maximum.
	Maximum,
}

impl Display for MaximumNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::MaximumNumber::*;

		match self
		{
			Finite(value) => write!(f, "{}", value),

			&Maximum => write!(f, "max"),
		}
	}
}

impl Default for MaximumNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		MaximumNumber::Maximum
	}
}

impl Into<Option<usize>> for MaximumNumber
{
	#[inline(always)]
	fn into(self) -> Option<usize>
	{
		use self::MaximumNumber::*;

		match self
		{
			Finite(value) => Some(value),
			Maximum => None,
		}
	}
}

impl From<Option<usize>> for MaximumNumber
{
	#[inline(always)]
	fn from(value: Option<usize>) -> Self
	{
		use self::MaximumNumber::*;

		match value
		{
			Some(value) => Finite(value),
			None => Maximum,
		}
	}
}

impl MaximumNumber
{
	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, MaximumNumberParseError>
	{
		let contents = read_to_string(file_path)?;
		Self::from_file_contents(contents)
	}

	#[inline(always)]
	fn from_file_contents(mut contents: String) -> Result<Self, MaximumNumberParseError>
	{
		if unlikely!(!contents.ends_with('\n'))
		{
			return Err(MaximumNumberParseError::DoesNotEndWithLineFeed)
		}
		contents.truncate(contents.len() - 1);

		use self::MaximumNumber::*;
		if &contents == "max"
		{
			Ok(Maximum)
		}
		else
		{
			let value: usize = contents.parse()?;
			Ok(Finite(value))
		}
	}
}
