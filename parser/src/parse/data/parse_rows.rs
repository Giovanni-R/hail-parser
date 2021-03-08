use nom::IResult;

use crate::types::{EType, HailValue};

use super::{DataParsingError, Encoding};

/// Decoding thedata means repeatedly decoding the value from the file until there are no elements
/// left.
///
/// Before each row, a boolean byte (`0u8`/`1u8`) indicates whether there is a new row or there are
/// no rows left.
pub fn parse_rows<E: Encoding>(
    i: &[u8],
    row_type: &EType,
) -> Result<Vec<HailValue>, DataParsingError> {
    match _parse_rows::<E>(i, row_type) {
        Ok((_, data)) => Ok(data),
        Err(e) => Err(DataParsingError::Generic(e.to_string())),
    }
}

fn _parse_rows<'i, E: Encoding>(
    i: &'i [u8],
    row_type: &EType,
) -> IResult<&'i [u8], Vec<HailValue>> {
    let mut result = Vec::new();
    let mut rest = i;
    while let (inner_rest, true) = E::bool(rest)? {
        let (inner_rest, parsed_value) = row_type.decode_from::<E>(inner_rest)?;
        rest = inner_rest;
        result.push(parsed_value);
    }
    Ok((rest, result))
}
