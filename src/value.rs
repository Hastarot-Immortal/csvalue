/// A wrapper for all common types a such as integer, float, string, etc.
///
/// For convenience, there are many `as_*` methods for basic types such as integers, floats, and strings. 
///
/// # Example
/// 
/// ```
/// use csvalue::Value;
/// 
/// let value = Value::from(42);
/// let not_int_value = Value::from("not u8");
/// let int = value.as_u8();
/// let not_int = not_int_value.as_u8();
/// 
/// assert_eq!(int, Some(42));
/// assert_eq!(not_int, None);
/// ```
#[cfg(feature="value")]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
  Int(i64),
  Float(f64),
  Str(String),
  #[default]
  None,
}

impl Value {
  pub fn as_u8(&self) -> Option<u8> {
    match *self {
      Self::Int(num) => u8::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_u16(&self) -> Option<u16> {
    match *self {
      Self::Int(num) => u16::try_from(num).ok(),
      _ => None,
    }
  }
  
  pub fn as_u32(&self) -> Option<u32> {
    match *self {
      Self::Int(num) => u32::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_u64(&self) -> Option<u64> {
    match *self {
      Self::Int(num) => u64::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_u128(&self) -> Option<u128> {
    match *self {
      Self::Int(num) => u128::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_usize(&self) -> Option<usize> {
    match *self {
      Self::Int(num) => usize::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_i8(&self) -> Option<i8> {
    match *self {
      Self::Int(num) => i8::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_i16(&self) -> Option<i16> {
    match *self {
      Self::Int(num) => i16::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_i32(&self) -> Option<i32> {
    match *self {
      Self::Int(num) => i32::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_i64(&self) -> Option<i64> {
    match *self {
      Self::Int(num) => Some(num),
      _ => None,
    }
  }

  pub fn as_i128(&self) -> Option<i128> {
    match *self {
      Self::Int(num) => i128::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_isize(&self) -> Option<isize> {
    match *self {
      Self::Int(num) => isize::try_from(num).ok(),
      _ => None,
    }
  }

  pub fn as_f32(&self) -> Option<f32> {
    match *self {
      Self::Float(num) => Some(num as f32),
      _ => None,
    }
  }

  pub fn as_f64(&self) -> Option<f64> {
    match *self {
      Self::Float(num) => Some(num),
      _ => None,
    }
  }

  pub fn as_str(&self) -> Option<&str> {
    match self {
      Self::Str(string) => Some(string.as_str()),
      _ => None,
    }
  }

  pub fn as_string(&self) -> Option<String> {
    match self {
      Self::Str(string) => Some(string.clone()),
      _ => None,
    }
  }
}

macro_rules! impl_from_num {
  ($($t: ty),*) => {
    $(
      impl From<$t> for Value {
        fn from(value: $t) -> Self {
          Self::Int(value as i64)
        }
      }
    )*
  };
}

impl_from_num!(i8, i16, i32, i64, isize, u8, u16, u32);

impl From<f32> for Value {
  fn from(value: f32) -> Self {
    Value::Float(value as f64)
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Value::Float(value)
  }
}

impl From<&str> for Value {
  fn from(value: &str) -> Self {
    Value::Str(value.to_owned())
  }
}

impl From<String> for Value {
  fn from(value: String) -> Self {
    Value::Str(value)
  }
}

macro_rules! impl_from_option {
  ($($t: ty),*) => {
    $(
      impl From<Option<$t>> for Value {
        fn from(value: Option<$t>) -> Self {
          match value {
            Some(value) => Value::from(value),
            None => Value::None,
          }
        }
      }
    )*
  };
}

impl_from_option!(
  i8, i16, i32, i64, isize,
  u8, u16, u32, f32, f64,
  &str, String
);

macro_rules! impl_try_from_num {
  ($($t: ty),*) => {
    $(
      impl TryFrom<$t> for Value {
        type Error = <i64 as TryFrom<$t>>::Error;
      
        fn try_from(value: $t) -> Result<Self, Self::Error> {
          value.try_into().map(|v| Self::Int(v))
        }
      }

      impl TryFrom<Option<$t>> for Value {
        type Error = <i64 as TryFrom<$t>>::Error;

        fn try_from(value: Option<$t>) -> Result<Self, Self::Error> {
          match value {
            Some(value) => Self::try_from(value),
            None => Ok(Value::None),
          }
        }
      }
    )*
  };
}

impl_try_from_num!(i128, u128, u64, usize);

