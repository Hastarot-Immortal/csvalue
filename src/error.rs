use std::{
	fmt::Display,
	error::Error,
	num::{ParseIntError, ParseFloatError},
};

#[cfg(feature="record")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
	UnterminatedStr,
	InvalidInt(ParseIntError),
	InvalidFloat(ParseFloatError),
	NoDelimiter,
}

impl Display for RecordError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", match self {
			Self::UnterminatedStr => "unterminated string".to_string(),
			Self::InvalidInt(err) => err.to_string(),
			Self::InvalidFloat(err) => err.to_string(),
			Self::NoDelimiter => "no delimiter between values".to_string(),
		})
	}
}

impl Error for RecordError {}
