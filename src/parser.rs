use std::io::Read;

pub trait Parser {
  type Output;
  type Error;

  fn parse(&self, content: &str) -> Result<Self::Output, Self::Error>;
  fn parse_reader(&self, reader: &mut impl Read) -> Result<Self::Output, Self::Error>;
}

pub trait Delimeter {
  type CharSeq;

  fn is_match(&self, seq: &mut Self::CharSeq) -> bool;
}

