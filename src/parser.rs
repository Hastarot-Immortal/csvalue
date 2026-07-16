use std::io::{Read, BufReader};

pub trait Parser {
  type Output;
  type Error;

  fn parse(&self, reader: &mut impl Read) -> Result<Self::Output, Self::Error>;

  fn parse_str(&self, content: &str) -> Result<Self::Output, Self::Error> {
    self.parse(&mut content.as_bytes())
  }

  fn parse_buf(&self, reader: impl Read) -> Result<Self::Output, Self::Error> {
    self.parse(&mut BufReader::new(reader))
  }
}

mod sealed {
  pub trait Delimeter {}

  impl Delimeter for char {}
}

pub trait Delimeter: sealed::Delimeter {
  type Buffer: Default;

  fn is_match(&self, seq: &mut Self::Buffer) -> bool;
  fn len(&self) -> usize;
}
