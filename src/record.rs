use crate::{Value, RecordError, Parser, Delimeter};
use std::{
  // iter::Peekable,
  // str::Chars,
  io::Read,
};

pub struct RecordParser<D: Delimeter> {
  has_header: bool,
  delimeter: D,
}

impl<D> Parser for RecordParser<D>
where
	D: Delimeter
{
	type Output = Records;
	type Error = RecordError;

  fn parse(&self, content: &str) -> Result<Self::Output, Self::Error> {
		todo!()
  }
  
  fn parse_reader(&self, reader: &mut impl Read) -> Result<Self::Output, Self::Error> {
		todo!()
  }
}

/// A dynamically-sized array of [`Value`].
/// 
/// Implements all methods of `Vec`.
pub type Record = Vec<Value>;

pub struct Records(Vec<Records>);
