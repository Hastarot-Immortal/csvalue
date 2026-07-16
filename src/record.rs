use crate::{
// Value, 
RecordError, Parser, Delimeter};
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

  fn parse(&self, reader: &mut impl Read) -> Result<Self::Output, Self::Error> {
    let mut records = Records::new();
    let mut row_pointer = 0u32;
    let mut buf = D::Buffer::default();
  
		Ok(records)
  }
}

// type CharIter<'a> = Peekable<Chars<'a>>;

type Record = (u8, u32);

pub struct Records {
    pool: ValuePool,
    records: Vec<Record>,
    rows: Vec<u32>,
}

impl Records {
  fn new() -> Self {
    Self {
      pool: ValuePool::new(),
      records: Vec::new(),
      rows: Vec::new(),
    }
  }
}

struct ValuePool {
  ints: Pool<i64>,
  floats: Pool<f64>,
  strings: Pool<String>,
}

impl ValuePool {
  fn new() -> Self {
    Self {
      ints: Pool(Vec::new()),
      floats: Pool(Vec::new()),
      strings: Pool(Vec::new()),
      
    }
  }
}

struct Pool<T>(Vec<T>);
