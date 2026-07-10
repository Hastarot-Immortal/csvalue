/// A wrapper for all common types a such as integer and float numbers, strings, etc.
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
  /// Convert [`Value::Int`] into `Option<u8>` without consuming `Value` itself.
  ///
  /// ```
  /// use csvalue::Value;
  /// 
  /// let value = Value::from(42);
  /// let int = value.as_u8();
  /// 
  /// assert_eq!(int, Some(42))
  /// ```
  pub fn as_u8(&self) -> Option<u8> {
    match *self {
      Self::Int(num) => u8::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<u16>` without consuming `Value` itself.
  pub fn as_u16(&self) -> Option<u16> {
    match *self {
      Self::Int(num) => u16::try_from(num).ok(),
      _ => None,
    }
  }
  
  /// Convert [`Value::Int`] into `Option<u32>` without consuming `Value` itself.
  pub fn as_u32(&self) -> Option<u32> {
    match *self {
      Self::Int(num) => u32::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<u64>` without consuming `Value` itself.
  pub fn as_u64(&self) -> Option<u64> {
    match *self {
      Self::Int(num) => u64::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<u128>` without consuming `Value` itself.
  pub fn as_u128(&self) -> Option<u128> {
    match *self {
      Self::Int(num) => u128::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<usize>` without consuming `Value` itself.
  pub fn as_usize(&self) -> Option<usize> {
    match *self {
      Self::Int(num) => usize::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<i8>` without consuming `Value` itself.
  pub fn as_i8(&self) -> Option<i8> {
    match *self {
      Self::Int(num) => i8::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<i16>` without consuming `Value` itself.
  pub fn as_i16(&self) -> Option<i16> {
    match *self {
      Self::Int(num) => i16::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<i32>` without consuming `Value` itself.
  pub fn as_i32(&self) -> Option<i32> {
    match *self {
      Self::Int(num) => i32::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<i64>` without consuming `Value` itself.
  pub fn as_i64(&self) -> Option<i64> {
    match *self {
      Self::Int(num) => i64::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<i128>` without consuming `Value` itself.
  pub fn as_i128(&self) -> Option<i128> {
    match *self {
      Self::Int(num) => i128::try_from(num).ok(),
      _ => None,
    }
  }

  /// Convert [`Value::Int`] into `Option<isize>` without consuming `Value` itself.
  pub fn as_isize(&self) -> Option<isize> {
    match *self {
      Self::Int(num) => isize::try_from(num).ok(),
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

