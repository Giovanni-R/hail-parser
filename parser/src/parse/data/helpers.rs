use nom::bytes::complete as bytes;
use nom::IResult;

use crate::{
    parse::Encoding,
    types::{EType, HailValue},
};

pub(crate) fn sequence<'i, E: Encoding>(
    i: &'i [u8],
    inner_type: &EType,
) -> IResult<&'i [u8], Vec<HailValue>> {
    match inner_type.required {
        true => sequence_without_check::<E>(i, inner_type),
        false => sequence_with_check::<E>(i, inner_type),
    }
}

fn sequence_with_check<'i, E: Encoding>(
    i: &'i [u8],
    inner_type: &EType,
) -> IResult<&'i [u8], Vec<HailValue>> {
    let (rest, len) = E::u32(i)?;

    if len == 0 {
        return Ok((rest, vec![]));
    }

    let (mut rest, presence_vec) = presence_array(rest, len as usize)?;
    let mut presence_iter = presence_vec.into_iter();

    let mut result: Vec<HailValue> = Vec::new();

    for _ in 0..len {
        match presence_iter.next() {
            Some(false) => {
                result.push(HailValue::Missing);
            }
            Some(true) => {
                let (inner_rest, decoded_type) = inner_type.decode_from::<E>(rest)?;
                rest = inner_rest;
                result.push(decoded_type);
            }
            // presence_iter shouldn't exaust before the lenth of the array.
            None => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::TagBits,
                )))
            }
        }
    }

    Ok((rest, result))
}

/// A Hail array declares its length at the beginning and then encodes each element without
/// spacing.
fn sequence_without_check<'i, E: Encoding>(
    i: &'i [u8],
    inner_type: &EType,
) -> IResult<&'i [u8], Vec<HailValue>> {
    let (rest, len) = E::u32(i)?;

    sequence_with_given_length_without_check::<E>(rest, inner_type, len.into())
}

pub(crate) fn sequence_with_given_length_without_check<'i, E: Encoding>(
    i: &'i [u8],
    inner_type: &EType,
    len: i64,
) -> IResult<&'i [u8], Vec<HailValue>> {
    let mut rest = i;

    let mut result: Vec<HailValue> = Vec::new();

    for _ in 0..len {
        let (inner_rest, decoded_type) = inner_type.decode_from::<E>(rest)?;
        rest = inner_rest;
        result.push(decoded_type);
    }

    Ok((rest, result))
}

/// A presence array is represented as an array of bytes, long enough to have a bit for each
/// field considered.
///
/// The bits are considered least-significant first, and first byte first.
/// [(8)(7)(6)(5)_(4)(3)(2)(1), (16)(15)(14)(13)_(12)(11)(10)(9)]
pub fn presence_array(i: &[u8], field_count: usize) -> IResult<&[u8], Vec<bool>> {
    // Compute how many bytes are needed to have one bit per field.
    let number_of_bytes_to_take = {
        let extra = field_count % 8;
        if extra == 0 {
            field_count / 8
        } else {
            1 + ((field_count - extra) / 8)
        }
    };

    let (rest, bytes) = bytes::take(number_of_bytes_to_take)(i)?;

    let mut are_present = Vec::<bool>::new();

    // The bytes are in order but bits must be read in reverse
    // (that is, the digit of lowest significance or right-most corresponds to earlier fields).
    for byte in bytes {
        are_present.push((byte & 0b_0000_0001) == 0);
        are_present.push((byte & 0b_0000_0010) == 0);
        are_present.push((byte & 0b_0000_0100) == 0);
        are_present.push((byte & 0b_0000_1000) == 0);
        are_present.push((byte & 0b_0001_0000) == 0);
        are_present.push((byte & 0b_0010_0000) == 0);
        are_present.push((byte & 0b_0100_0000) == 0);
        are_present.push((byte & 0b_1000_0000) == 0);
    }

    if are_present.len() < field_count {
        return Err(nom::Err::Failure(nom::error::Error::new(
            i,
            nom::error::ErrorKind::TagBits,
        )));
    }

    are_present.resize(field_count, false);

    Ok((rest, are_present))
}
