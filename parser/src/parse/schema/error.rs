use std::fmt::{self, Display};

/// This is a bare-bones error implementation. Taken from the basic example in the docs.
#[derive(Clone, Debug, PartialEq)]
pub enum SchemaParsingError {
    Generic(String),
}

impl serde::ser::Error for SchemaParsingError {
    fn custom<T: Display>(msg: T) -> Self {
        SchemaParsingError::Generic(msg.to_string())
    }
}

impl serde::de::Error for SchemaParsingError {
    fn custom<T: Display>(msg: T) -> Self {
        SchemaParsingError::Generic(msg.to_string())
    }
}

impl Display for SchemaParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaParsingError::Generic(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for SchemaParsingError {}
