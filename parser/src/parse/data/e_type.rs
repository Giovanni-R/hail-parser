use nom::IResult;

use crate::types::{encoding::VirtualHint, EType, ETypeShape, HailValue};

use super::{encoders::Encoding, parse};

impl EType {
    pub fn decode_from<'i, E: Encoding>(&self, i: &'i [u8]) -> IResult<&'i [u8], HailValue> {
        match self {
            // [Struct]
            EType {
                shape: ETypeShape::BaseStruct(ref inner_mapping),
                virtual_hint: None,
                ..
            } => parse::hail_struct::<E>(i, inner_mapping),
            // Locus as Struct
            EType {
                shape: ETypeShape::BaseStruct(_),
                virtual_hint: Some(VirtualHint::Locus(genome)),
                ..
            } => parse::locus::<E>(i, &genome),
            // Interval as Struct
            EType {
                shape: ETypeShape::BaseStruct(ref inner_mapping),
                virtual_hint: Some(VirtualHint::Interval),
                ..
            } => parse::interval::<E>(i, inner_mapping),
            // Tuple as Struct
            EType {
                shape: ETypeShape::BaseStruct(ref inner_mapping),
                virtual_hint: Some(VirtualHint::Tuple),
                ..
            } => parse::tuple::<E>(i, inner_mapping),

            // [Array]
            EType {
                shape: ETypeShape::Array(ref inner_type),
                virtual_hint: None,
                ..
            } => parse::array::<E>(i, inner_type),
            // Set as Array
            EType {
                shape: ETypeShape::Array(ref inner_type),
                virtual_hint: Some(VirtualHint::Set),
                ..
            } => parse::set::<E>(i, inner_type),
            // Dict as Array
            EType {
                shape: ETypeShape::Array(ref inner_type),
                virtual_hint: Some(VirtualHint::Dict),
                ..
            } => parse::dict::<E>(i, inner_type),

            // NDArray as NDArrayColumnMajor
            EType {
                shape: ETypeShape::NdArrayColumnMajor(ref inner_type, n),
                virtual_hint: None,
                ..
            } => parse::ndarray_column_major::<E>(i, inner_type, *n),

            // String as Binary
            EType {
                shape: ETypeShape::Binary,
                virtual_hint: Some(VirtualHint::String),
                ..
            } => parse::string::<E>(i),

            // [Float32]
            EType {
                shape: ETypeShape::Float32,
                virtual_hint: None,
                ..
            } => parse::f32::<E>(i),

            // [Float64]
            EType {
                shape: ETypeShape::Float64,
                virtual_hint: None,
                ..
            } => parse::f64::<E>(i),

            // [Int32]
            EType {
                shape: ETypeShape::Int32,
                virtual_hint: None,
                ..
            } => parse::u32::<E>(i),

            // [Int64]
            EType {
                shape: ETypeShape::Int64,
                virtual_hint: None,
                ..
            } => parse::i64::<E>(i),

            // Int32 as Call
            EType {
                shape: ETypeShape::Int32,
                virtual_hint: Some(VirtualHint::Call),
                ..
            } => parse::call::<E>(i),

            // [Boolean]
            EType {
                shape: ETypeShape::Boolean,
                virtual_hint: None,
                ..
            } => parse::bool::<E>(i),

            // Anything else
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                i,
                nom::error::ErrorKind::ParseTo,
            ))),
        }
    }
}
