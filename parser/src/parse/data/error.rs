use std::fmt::{self, Display};

/// This is a bare-bones error implementation. Taken from the basic example in the docs.
#[derive(Clone, Debug, PartialEq)]
pub enum DataParsingError {
    Generic(String),
}

impl Display for DataParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataParsingError::Generic(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for DataParsingError {}
