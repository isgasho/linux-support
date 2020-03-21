/// A parse error.
#[derive(Debug)]
pub enum ProcessStatusStatisticParseError
{
	/// No value.
	NoValue,

	/// Value was not preceeded by a horizontal tab.
	ValueNotPreceededByHorizontalTab,

	/// Length was invalid.
	InvalidLength,

	/// Ending was invalid.
	InvalidEnding,

	/// Separator of components of value was invalid in some way; either not present, the wrong kind or too few or too many.
	InvalidSeparator,

	/// Value was out-of-range, eg `2` for a `bool`.
	OutOfRange,

	/// Statistic was present more than once.
	DuplicatedStatistic,

	/// Statistic value sub-set had a duplicated entry.
	DuplicatedStatisticValue,

	/// Value was not a valid UTF-8 string.
	NotAUtf8String(Utf8Error),

	/// Value was not a valid integer.
	NotAValidInteger(ParseIntError),

	/// Value was not a valid CPU or NUMA node list.
	NotAValidListOfCpusOrNumaNodes(ListParseError),
}

impl Display for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessStatusStatisticParseError::*;

		match self
		{
			&NoValue => None,

			&ValueNotPreceededByHorizontalTab => None,

			&InvalidLength => None,

			&InvalidEnding => None,

			&InvalidSeparator => None,

			&OutOfRange => None,

			&DuplicatedStatistic => None,

			&DuplicatedStatisticValue => None,

			&NotAUtf8String(ref error) => Some(error),

			&NotAValidInteger(ref error) => Some(error),

			&NotAValidListOfCpusOrNumaNodes(ref error) => Some(error),
		}
	}
}

impl From<Utf8Error> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: Utf8Error) -> Self
	{
		ProcessStatusStatisticParseError::NotAUtf8String(error)
	}
}

impl From<ParseIntError> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: ParseIntError) -> Self
	{
		ProcessStatusStatisticParseError::NotAValidInteger(error)
	}
}

impl From<ListParseError> for ProcessStatusStatisticParseError
{
	#[inline(always)]
	fn from(error: ListParseError) -> Self
	{
		ProcessStatusStatisticParseError::NotAValidListOfCpusOrNumaNodes(error)
	}
}
