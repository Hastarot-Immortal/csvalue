//! A simple and small library for parsing the contents of CSV files.

#[cfg(feature="value")]
pub mod value;
#[cfg(feature="parser")]
pub mod parser;
#[cfg(feature="record")]
pub mod record;
pub mod error;

#[cfg(feature="value")]
pub use value::Value;
#[cfg(feature="parser")]
pub use parser::{Parser, Delimeter};
#[cfg(feature="record")]
pub use record::{Records, RecordParser};
#[cfg(feature="record")]
pub use error::RecordError;

