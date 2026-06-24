#[cfg(feature="value")]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
  Int(i64),
  Float(f64),
  Str(String),
  #[default]
  None,
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

impl_from_num!(i8, i16, i32, i64, isize);
impl_from_num!(u8, u16, u32);

impl From<f32> for Value {
  fn from(value: f32) -> Self {
    Value::Float(value as f64)
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Value::Float(value as f64)
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

impl_from_option!(i8, i16, i32, i64, isize);
impl_from_option!(u8, u16, u32);
impl_from_option!(f32, f64);
impl_from_option!(&str, String);

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
