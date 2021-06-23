use nom::error::{Error, ErrorKind};
use nom::Err::Failure;
use nom::{alt, named, tag, IResult};

use crate::types::{EType, ETypeShape, VType, VTypeShape};

use super::error::SchemaParsingError;

/// A small trait to help generalise over the virtual and encoded schemas,
/// as they are extremelly similar.
///
/// It also handles error conversion from the nom error.
pub trait SchemaFromString: Sized {
    fn parse_type(schema: &str) -> Result<Self, SchemaParsingError> {
        let result = Self::take_type(schema);

        match result {
            Ok((_, schema)) => Ok(schema),
            Err(nom::Err::Failure(nom::error::Error { input, code })) => {
                Err(SchemaParsingError::Generic(format!(
                    "Unable to parse the schema: {}\nParsing error: {:?}\nLocation: {}",
                    &schema, code, input,
                )))
            }
            Err(e) => Err(SchemaParsingError::Generic(format!(
                "Unable to parse the schema: {}\nParsing error: {}",
                &schema,
                e.to_string()
            ))),
        }
    }

    fn parse_named_type(schema: &str) -> Result<(String, Self), SchemaParsingError> {
        let result = helpers::take_field::<Self>(schema);

        match result {
            Ok((_, named_field)) => Ok(named_field),
            Err(nom::Err::Failure(nom::error::Error { input, code })) => {
                Err(SchemaParsingError::Generic(format!(
                    "Unable to parse the schema: {}\nParsing error: {:?}\nLocation: {}",
                    &schema, code, input,
                )))
            }
            Err(e) => Err(SchemaParsingError::Generic(format!(
                "Unable to parse the schema: {}\nParsing error: {}",
                &schema,
                e.to_string()
            ))),
        }
    }

    fn take_type(i: &str) -> IResult<&str, Self>;
}

impl SchemaFromString for VType {
    /// Parses a [VType] recursively.
    fn take_type(i: &str) -> IResult<&str, Self> {
        // This expands to a function that takes one of the tags from the input.
        named!(take_virtual_type_root<&str,&str>,
            alt!(
                tag!("Struct") | tag!("Tuple") | tag!("Dict") | tag!("Interval") |
                tag!("Array") | tag!("Set") | tag!("NDArray") |
                tag!("String") | tag!("Boolean") |
                tag!("Float32") | tag!("Float64") | tag!("Int32") | tag!("Int64") |
                tag!("Locus") | tag!("Call")
            )
        );

        let (rest, required) = helpers::is_required_field(i)?;

        let (mut rest, type_root) = take_virtual_type_root(rest)?;
        let shape = match type_root {
            "Struct" => {
                let (inner_rest, field_sequence) = helpers::take_struct(rest)?;
                rest = inner_rest;
                VTypeShape::Struct(field_sequence)
            }
            "Tuple" => {
                let (inner_rest, field_sequence) = helpers::take_tuple_types(rest)?;
                rest = inner_rest;
                VTypeShape::Tuple(field_sequence)
            }
            "Array" => {
                let (inner_rest, element_type) =
                    helpers::take_single_type_in_square_brackets(rest)?;
                rest = inner_rest;
                VTypeShape::Array(Box::new(element_type))
            }
            "Set" => {
                let (inner_rest, element_type) =
                    helpers::take_single_type_in_square_brackets(rest)?;
                rest = inner_rest;
                VTypeShape::Array(Box::new(element_type))
            }
            "Dict" => {
                let (inner_rest, (key_type, value_type)) = helpers::take_dict_types(rest)?;
                rest = inner_rest;
                VTypeShape::Dict(Box::new(key_type), Box::new(value_type))
            }
            "Interval" => {
                let (inner_rest, element_type) =
                    helpers::take_single_type_in_square_brackets(rest)?;
                rest = inner_rest;
                VTypeShape::Interval(Box::new(element_type))
            }
            "NDArray" => {
                let (inner_rest, (element_type, n)) =
                    helpers::take_ndarray_type_and_dimensionality(rest)?;
                rest = inner_rest;
                VTypeShape::NDArray(Box::new(element_type), n)
            }
            "String" => VTypeShape::String,
            "Float32" => VTypeShape::Float32,
            "Float64" => VTypeShape::Float64,
            "Int32" => VTypeShape::Int32,
            "Int64" => VTypeShape::Int64,
            "Boolean" => VTypeShape::Boolean,
            "Locus" => {
                let (inner_rest, genome) = helpers::take_round_brackets_content(rest)?;
                rest = inner_rest;
                VTypeShape::Locus(genome.into())
            }
            "Call" => VTypeShape::Call,
            _ => return Err(Failure(Error::new(rest, ErrorKind::Not))),
        };

        Ok((rest, VType { shape, required }))
    }
}

impl SchemaFromString for EType {
    /// Parses a [EType] recursively.
    fn take_type(i: &str) -> IResult<&str, Self> {
        // This expands to a function that takes one of the tags from the input.
        named!(take_encoded_type_root<&str,&str>,
            alt!(
                tag!("EBaseStruct") |
                tag!("EArray") | tag!("ENDArrayColumnMajor") |
                tag!("EBinary") | tag!("EBoolean") |
                tag!("EFloat32") | tag!("EFloat64") | tag!("EInt32") | tag!("EInt64")
            )
        );

        let (rest, required) = helpers::is_required_field(i)?;

        let (mut rest, type_root) = take_encoded_type_root(rest)?;
        let shape = match type_root {
            "EBaseStruct" => {
                let (inner_rest, field_sequence) = helpers::take_struct(rest)?;
                rest = inner_rest;
                ETypeShape::BaseStruct(field_sequence)
            }
            "EArray" => {
                let (inner_rest, element_type) =
                    helpers::take_single_type_in_square_brackets(rest)?;
                rest = inner_rest;
                ETypeShape::Array(Box::new(element_type))
            }
            "ENDArrayColumnMajor" => {
                let (inner_rest, (element_type, n)) =
                    helpers::take_ndarray_type_and_dimensionality(rest)?;
                rest = inner_rest;
                ETypeShape::NdArrayColumnMajor(Box::new(element_type), n)
            }
            "EBinary" => ETypeShape::Binary,
            "EFloat32" => ETypeShape::Float32,
            "EFloat64" => ETypeShape::Float64,
            "EInt32" => ETypeShape::Int32,
            "EInt64" => ETypeShape::Int64,
            "EBoolean" => ETypeShape::Boolean,
            _ => return Err(Failure(Error::new(rest, ErrorKind::Not))),
        };

        Ok((
            rest,
            EType {
                shape,
                required,
                virtual_hint: None,
            },
        ))
    }
}

/// This module holds the shared logic between the virtual and encoded schema parsing.
mod helpers {
    use nom::{is_a, named, tag, take, take_until1, IResult};

    use super::SchemaFromString;

    /// Takes a named type of the form "name:Type".
    pub fn take_field<T: SchemaFromString + Sized>(i: &str) -> IResult<&str, (String, T)> {
        let (rest, name) = take_field_name(i)?;

        let (rest, _) = take_colon(rest)?; // :

        let (rest, encoded_type) = T::take_type(rest)?;

        Ok((rest, (name.into(), encoded_type)))
    }

    pub fn take_struct<T: SchemaFromString + Sized>(i: &str) -> IResult<&str, Vec<(String, T)>> {
        let (rest, _) = take_curly_bracket_open(i)?; // {

        let (rest, fields) = take_field_sequence::<T>(rest)?;

        let (rest, _) = take_curly_bracket_close(rest)?; // }

        Ok((rest, fields))
    }

    pub fn take_field_sequence<T: SchemaFromString + Sized>(
        i: &str,
    ) -> IResult<&str, Vec<(String, T)>> {
        let mut fields: Vec<(String, T)> = Vec::new();

        let mut rest = i;

        while let (inner_rest, true) = do_continue_sequence_close_curly(rest)? {
            let (inner_rest, field) = take_field::<T>(inner_rest)?;
            rest = inner_rest;
            fields.push(field);
        }

        Ok((rest, fields))
    }

    pub fn take_tuple_types<T: SchemaFromString + Sized>(i: &str) -> IResult<&str, Vec<T>> {
        let (rest, _) = take_square_bracket_open(i)?; // [

        let (rest, fields) = take_type_sequence(rest)?;

        let (rest, _) = take_square_bracket_close(rest)?; // ]

        Ok((rest, fields))
    }

    pub fn take_type_sequence<T: SchemaFromString + Sized>(i: &str) -> IResult<&str, Vec<T>> {
        let mut fields: Vec<T> = Vec::new();

        let mut rest = i;

        while let (inner_rest, true) = do_continue_sequence_close_square(rest)? {
            let (inner_rest, field) = T::take_type(inner_rest)?;
            rest = inner_rest;
            fields.push(field);
        }

        Ok((rest, fields))
    }

    pub fn take_single_type_in_square_brackets<T: SchemaFromString + Sized>(
        i: &str,
    ) -> IResult<&str, T> {
        let (rest, _) = take_square_bracket_open(i)?; // [

        let (rest, element_type) = T::take_type(rest)?;

        let (rest, _) = take_square_bracket_close(rest)?; // ]

        Ok((rest, element_type))
    }

    pub fn take_dict_types<T: SchemaFromString + Sized>(i: &str) -> IResult<&str, (T, T)> {
        let (rest, _) = take_square_bracket_open(i)?; // [

        let (rest, key_type) = T::take_type(rest)?;

        let (rest, _) = take_comma(rest)?; // ,

        let (rest, value_type) = T::take_type(rest)?;

        let (rest, _) = take_square_bracket_close(rest)?; // ]

        Ok((rest, (key_type, value_type)))
    }

    pub fn take_ndarray_type_and_dimensionality<T: SchemaFromString + Sized>(
        i: &str,
    ) -> IResult<&str, (T, u32)> {
        let (rest, _) = take_square_bracket_open(i)?; // [

        let (rest, inner_type) = T::take_type(rest)?;

        let (rest, _) = take_comma(rest)?; // ,

        let (rest, digits) = take_digits(rest)?;
        let n: u32 = match digits.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::ParseTo,
                )))
            }
        };

        let (rest, _) = take_square_bracket_close(rest)?; // ]

        Ok((rest, (inner_type, n)))
    }

    pub fn take_round_brackets_content(i: &str) -> IResult<&str, &str> {
        let (rest, _) = take_round_bracket_open(i)?; // (

        let (rest, genome) = up_to_closing_round_bracket(rest)?;

        let (rest, _) = take_round_bracket_close(rest)?; // )

        Ok((rest, genome))
    }

    pub fn is_required_field(i: &str) -> IResult<&str, bool> {
        let (rest, maybe_plus) = take1(i)?;
        match maybe_plus {
            "+" => Ok((rest, true)),
            _ => Ok((i, false)),
        }
    }

    fn do_continue_sequence_close_curly(i: &str) -> IResult<&str, bool> {
        let (rest, next) = take1(i)?;
        match next {
            "}" => Ok((i, false)),
            "," => Ok((rest, true)),
            _ => Ok((i, true)),
        }
    }

    fn do_continue_sequence_close_square(i: &str) -> IResult<&str, bool> {
        let (rest, next) = take1(i)?;
        match next {
            "]" => Ok((i, false)),
            "," => Ok((rest, true)),
            _ => Ok((i, true)),
        }
    }

    named!(take1<&str,&str>, take!(1));
    named!(take_field_name<&str,&str>, take_until1!(":"));
    named!(take_colon<&str,&str>, tag!(":"));
    named!(take_comma<&str,&str>, tag!(","));

    named!(take_curly_bracket_open<&str,&str>, tag!("{"));
    named!(take_curly_bracket_close<&str,&str>, tag!("}"));
    named!(take_square_bracket_open<&str,&str>, tag!("["));
    named!(take_square_bracket_close<&str,&str>, tag!("]"));
    named!(take_round_bracket_open<&str,&str>, tag!("("));
    named!(take_round_bracket_close<&str,&str>, tag!(")"));

    named!(up_to_closing_round_bracket<&str,&str>, take_until1!(")"));

    named!(take_digits<&str,&str>, is_a!("0123456789"));
}
