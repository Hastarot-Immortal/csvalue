use crate::error::ShortStrError;

use std::{
  fmt::{Debug, Display, Formatter, Result as FmtResult},
  ops::Deref,
};

pub const MAX_LENGTH: usize = 23;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortStr {
  length: u8,
  storage: [u8; MAX_LENGTH],
}

impl ShortStr {
  pub fn as_str(&self) -> &str {
    std::str::from_utf8(&self.storage[..self.length as usize]).unwrap_or("")
  }
}

impl TryFrom<&str> for ShortStr {
  type Error = ShortStrError;
  
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.len() > MAX_LENGTH {
      return Err(ShortStrError::TooBigString);
    }
    
    Ok(Self {
      length: value.len() as u8,
      storage: {
        let mut arr = [0u8; MAX_LENGTH];
        arr[..value.len()].copy_from_slice(value.as_bytes());
        arr
      }
    })
  }
}

impl Debug for ShortStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for ShortStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for ShortStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for ShortStr {
    fn as_ref(&self) -> &[u8] {
        &self.storage[..self.length as usize]
    }
}

impl Deref for ShortStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
