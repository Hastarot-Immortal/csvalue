use std::{
	fmt::{Display, Formatter, Error as FmtError},
	error::Error,
	num::{ParseIntError, ParseFloatError},
};

#[cfg(feature="record")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
	UnterminatedStr,
	InvalidInt(ParseIntError),
	InvalidFloat(ParseFloatError),
	NoDelimiter,
}

impl Display for RecordError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(f, "{}", match self {
			Self::UnterminatedStr => "unterminated string".to_string(),
			Self::InvalidInt(err) => err.to_string(),
			Self::InvalidFloat(err) => err.to_string(),
			Self::NoDelimiter => "no delimiter between values".to_string(),
		})
	}
}

impl Error for RecordError {}

#[cfg(feature="short_str")]
pub enum ShortStrError {
	TooBigString,
}
