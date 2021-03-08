use std::fmt::{self, Display};

use serde::{de, ser};

use super::de::look_ahead::StructureNode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Custom(String),

    /// An error occurred in the LookAheadDeserializer before the actual deserialization could
    /// start.
    LookAheadError(String),

    /// This error is for the conversion from u32 to usize when reading a length.
    /// This error should never be raised on systems with 32 or 64 bit usize, even on systems with
    /// smaller usize it should only be raised if the read value overflows.
    InvalidLength(u32),

    /// When deserializing an option, the code will rely on the parent sequences having pushed to
    /// a stack a hint of whether the value is Some/None (present/absent).
    /// This error is raised if that stack is found to be empty.
    NoOptionFlag(StructureNode),

    /// The method deserialize_seq expects a StructureNode::VariableLengthSequence to be the
    /// structure parsed by the LookAheadDeserializer.
    ExpectedVariableLengthSequence(StructureNode),
    /// The method deserialize_tuple expects a StructureNode::GivenLengthSequence to be the
    /// structure parsed by the LookAheadDeserializer.
    ExpectedGivenLengthSequence(StructureNode, usize),
    /// The method deserialize_map expects a StructureNode::Map to be the
    /// structure parsed by the LookAheadDeserializer.
    ExpectedMap(StructureNode),

    /// A GivenLengthSequence may be a tuple, a struct, or a NDArray.
    /// This requires some care with the interplay between declared length and the number of fields
    /// present in the StructureNode::GivenLengthSequence.
    /// In general there should be a field for each element, but in the case of NDArrays (which do
    /// not have a length known before they are parsed) the LookAheadDeserializer always
    /// deserializes a single element because the NDArray is homogeneous.
    /// This error is raised if an unexpected combination of number-of-fields and sequence-length
    /// is found.
    UnexpectedGivenLengthSequenceLayout(StructureNode, usize),

    /// These types are unsupported because they are not part of the Hail data model.
    UnsupportedType,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        let message: String = match self {
            Custom(msg) => msg.to_owned(),

            LookAheadError(msg) => {
                format!("an error occurred in the LookAheadDeserializer: {}", msg)
            }

            InvalidLength(len) => format!("an invalid length was parsed ({})", len),

            NoOptionFlag(shape) => {
                format!("no option flags remaining while deserializing {:?}", shape)
            }

            ExpectedVariableLengthSequence(shape) => {
                format!("{:?} was found instead of a VariableLengthSequence", shape)
            }
            ExpectedGivenLengthSequence(shape, len) => format!(
                "{:?} was found instead of a GivenLengthSequence of length {}",
                shape, len
            ),
            ExpectedMap(shape) => format!("{:?} was found instead of a Map", shape),

            UnexpectedGivenLengthSequenceLayout(shape, len) => format!(
                "incompatible length and shape for GivenLengthSequence received: {:?} of length {}",
                shape, len
            ),

            UnsupportedType => "the type contains an invalid Hail type".to_owned(),
        };

        formatter.write_str(&message)
    }
}

impl std::error::Error for Error {}

impl From<nom::Err<nom::error::Error<&[u8]>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        Error::Custom(e.to_string())
    }
}
