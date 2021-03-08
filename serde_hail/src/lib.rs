#![forbid(unsafe_code)]
mod de;
mod error;

mod model_generation;

pub mod load;
pub mod types;

pub use de::{parse_rows, Deserializer};
pub use error::{Error, Result};
pub use model_generation::encoded_type_to_rust_type;
