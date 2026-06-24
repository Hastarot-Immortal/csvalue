#[cfg(feature="value")]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
  Int(i64),
  Float(f64),
  Str(String),
  #[default]
  None,
}
