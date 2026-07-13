//! A simple and small library for parsing the contents of CSV files.

#[cfg(feature="value")]
pub mod value;
#[cfg(feature="record")]
pub mod record;
pub mod error;
#[cfg(feature="short_str")]
pub mod short_str;

#[cfg(feature="value")]
pub use value::Value;
#[cfg(feature="record")]
pub use record::{Record, parse_records};
#[cfg(feature="record")]
pub use error::RecordError;
#[cfg(feature="short_str")]
pub use short_str::ShortStr;

