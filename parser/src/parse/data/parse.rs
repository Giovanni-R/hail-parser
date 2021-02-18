use std::collections::BTreeMap;

use nom::IResult;

use ndarray::{ArrayD, IxDyn, ShapeBuilder};

use crate::types::{EType, HailValue};

use super::encoders::Encoding;

/// A [HailValue::Struct] is a named map of values, much like a Rust struct.
/// The values are encoded in order, without the names.
/// Before each struct, an array of bites indicates whether each optional field is present using a
/// bit-flag.
///
/// If fields two and three are optional, and three is missing (+ means required):
/// ```ignore
/// [bit-flags][+field 1][field 2][field 3][+field 4]
/// [0000_0010][ value 1][value 2]         [ value 4]
/// ```
pub fn hail_struct<'i, E: Encoding>(
    i: &'i [u8],
    mapping: &[(String, EType)],
) -> IResult<&'i [u8], HailValue> {
    // Only the optional fields are included in the bit flags.
    let number_of_optional_fields = mapping.iter().filter(|(_, t)| !t.required).count();
    let (mut rest, are_present) = helpers::presence_array(i, number_of_optional_fields)?;

    let mut output = BTreeMap::<String, HailValue>::new();

    let mut presence_iter = are_present.into_iter();

    for (field_name, field_type) in mapping {
        if !field_type.required {
            match presence_iter.next() {
                Some(false) => {
                    output.insert(field_name.to_owned(), HailValue::Missing);
                    continue;
                }
                None => {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        i,
                        nom::error::ErrorKind::TagBits,
                    ))); // It should never run out of bits.
                }
                Some(true) => {} // pass: parse the value ↓
            }
        }
        let (inner_rest, decoded) = field_type.decode_from::<E>(rest)?;
        rest = inner_rest;
        output.insert(field_name.into(), decoded);
    }

    Ok((rest, HailValue::Struct(output)))
}

/// A tuple is simply a struct from which we discard all the field names and maintain the order
/// of the values.
pub fn tuple<'i, E: Encoding>(
    i: &'i [u8],
    mapping: &[(String, EType)],
) -> IResult<&'i [u8], HailValue> {
    let number_of_not_required_fields = mapping.iter().filter(|(_, t)| !t.required).count();
    let (mut rest, are_present) = helpers::presence_array(i, number_of_not_required_fields)?;

    let mut output = Vec::<HailValue>::new();

    let mut presence_iter = are_present.into_iter();

    // Note that for a tuple only the positional information is relevant,
    // so this assumes that the metadata contains the fields in order.
    for (_, field_type) in mapping {
        if !field_type.required {
            match presence_iter.next() {
                Some(false) => {
                    output.push(HailValue::Missing);
                    continue;
                }
                None => {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        i,
                        nom::error::ErrorKind::TagBits,
                    ))); // It should never run out of bits.
                }
                Some(true) => {} // pass: parse the value ↓
            }
        }
        let (inner_rest, decoded) = field_type.decode_from::<E>(rest)?;
        rest = inner_rest;
        output.push(decoded);
    }

    Ok((rest, HailValue::Tuple(output)))
}

/// A [HailValue::Array] is a sequence of values with the length of the sequence itself is
/// prepended to the data.
/// If the internal array type is not required, then the array also has presence array covering
/// each value.
pub fn array<'i, E: Encoding>(i: &'i [u8], inner_type: &EType) -> IResult<&'i [u8], HailValue> {
    let (rest, sequence) = helpers::sequence::<E>(i, inner_type)?;
    Ok((rest, HailValue::Array(sequence)))
}

/// A [HailValue::Set] is essentially an Array, with the exception that the recovered values
/// should be unique.
/// [HailValue::Missing] is also a valid value in the set.
pub fn set<'i, E: Encoding>(i: &'i [u8], inner_type: &EType) -> IResult<&'i [u8], HailValue> {
    let (rest, sequence) = helpers::sequence::<E>(i, inner_type)?;
    Ok((rest, HailValue::Set(sequence)))
}

/// A [HailValue::Dict] is an array of [ETypeShape::BaseStruct], where each struct has a key and
/// a value.
pub fn dict<'i, E: Encoding>(i: &'i [u8], inner_type: &EType) -> IResult<&'i [u8], HailValue> {
    // Could be simplified by unpacking the inner_type and parsing the key and value types directly.
    // Will skip for now however, to keep the parser close to the EType structure.
    let (rest, sequence) = helpers::sequence::<E>(i, inner_type)?;

    let mut dict = BTreeMap::new();

    for key_value_struct in sequence {
        match key_value_struct {
            HailValue::Struct(inner_map) => {
                let mut key_and_value_iter = inner_map.into_iter();

                // We should always find a key and a value, in this order because it's a BTreeMap.
                if let Some((_, key)) = key_and_value_iter.next() {
                    if let Some((_, value)) = key_and_value_iter.next() {
                        dict.insert(key, value);
                        continue;
                    };
                };

                return Err(nom::Err::Failure(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::NoneOf,
                )));
            }
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::ParseTo,
                )));
            }
        }
    }
    Ok((rest, HailValue::Dict(dict)))
}

/// A [HailValue::Interval] is a [ETypeShape::BaseStruct] with two bounds of the same type, and two
/// boolean flags to indicate whether the edges of the interval are included.
pub fn interval<'i, E: Encoding>(
    i: &'i [u8],
    struct_mapping: &[(String, EType)],
) -> IResult<&'i [u8], HailValue> {
    let (rest, raw_interval) = hail_struct::<E>(i, struct_mapping)?;

    match raw_interval {
        HailValue::Struct(inner_map) => {
            let mut iter = inner_map.into_iter();

            // We should always find the values in this order beacuse a struct is a BTree.
            if let Some((_, end)) = iter.next() {
                if let Some((_, HailValue::Boolean(includes_end))) = iter.next() {
                    if let Some((_, HailValue::Boolean(includes_start))) = iter.next() {
                        if let Some((_, start)) = iter.next() {
                            return Ok((
                                rest,
                                HailValue::Interval {
                                    start: Box::new(start),
                                    end: Box::new(end),
                                    includes_start,
                                    includes_end,
                                },
                            ));
                        }
                    }
                }
            }

            Err(nom::Err::Failure(nom::error::Error::new(
                i,
                nom::error::ErrorKind::NoneOf,
            )))
        }
        _ => Err(nom::Err::Failure(nom::error::Error::new(
            i,
            nom::error::ErrorKind::ParseTo,
        ))),
    }
}

/// A [HailValue::NDArray] is encoded as a sequence of d u64 integers where each one represents
/// the size of the array on an axis (d=number of dimensions).
///
/// This is followed up by a sequence of elements layed out in column major order (or f-order),
/// as suggested by the name.
pub fn ndarray_column_major<'i, E: Encoding>(
    i: &'i [u8],
    inner_type: &EType,
    n: u32,
) -> IResult<&'i [u8], HailValue> {
    // Get the dimensions along each axis.
    let mut dims = Vec::new();
    let mut rest = i;
    for _ in 0..n {
        let (inner_rest, d) = E::i64(rest)?;
        rest = inner_rest;
        dims.push(d);
    }

    // Overflow will cause a panic.
    let number_of_elements = dims.iter().product();

    // Assumes the inner type is required.
    let (rest, elements) = helpers::sequence_with_given_length_without_check::<E>(
        rest,
        inner_type,
        number_of_elements,
    )?;

    let maybe_array = {
        // This is safe, but will cause loss of data if any of the dimensions are more than the
        // native usize.
        // "The size of this primitive is how many bytes it takes to reference any location in
        // memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target,
        // this is 8 bytes."
        let size_dims: Vec<usize> = dims.iter().map(|d| *d as usize).collect();

        // Here we tell ndarray the shape of our data.
        // Note that by default ndarray uses a c-order (row-major) while Hail uses
        // a f-order (column-major), so we need to set it.
        //                      ↓↓↓ This is what we want.
        // row-major(c)    column-major(f)
        // [[1, 2, 3],     [[1, 4, 7],
        //  [4, 5, 6],      [2, 5, 8],
        //  [7, 8, 9]]      [3, 6, 9]]
        let shape = IxDyn(size_dims.as_slice()).f();

        ArrayD::from_shape_vec(shape, elements)
    };

    match maybe_array {
        Ok(array) => Ok((rest, HailValue::NdArray(array))),
        Err(_) => Err(nom::Err::Failure(nom::error::Error::new(
            i,
            nom::error::ErrorKind::ParseTo,
        ))),
    }
}

/// A [HailValue::Locus] is a string and a number in succession with no spacing.
pub fn locus<'i, E: Encoding>(i: &'i [u8], reference_genome: &str) -> IResult<&'i [u8], HailValue> {
    let (rest, contig) = E::string(i)?;
    let (rest, position) = E::u32(rest)?;
    Ok((
        rest,
        HailValue::Locus {
            contig,
            position,
            reference: reference_genome.into(),
        },
    ))
}

/// A [HailValue::Call] is represented by a u32.
pub fn call<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, one) = E::u32(i)?;
    Ok((rest, HailValue::Call(one)))
}

pub fn string<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, s) = E::string(i)?;
    Ok((rest, HailValue::String(s)))
}

pub fn f32<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, float) = E::f32(i)?;
    if float.is_nan() {
        Ok((rest, HailValue::Missing))
    } else {
        Ok((rest, HailValue::Float32(float)))
    }
}

pub fn f64<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, float) = E::f64(i)?;
    if float.is_nan() {
        Ok((rest, HailValue::Missing))
    } else {
        Ok((rest, HailValue::Float64(float)))
    }
}

pub fn u32<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, int) = E::u32(i)?;
    Ok((rest, HailValue::Int32(int)))
}

pub fn i64<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, int) = E::i64(i)?;
    Ok((rest, HailValue::Int64(int)))
}

pub fn bool<E: Encoding>(i: &[u8]) -> IResult<&[u8], HailValue> {
    let (rest, flag) = E::bool(i)?;
    Ok((rest, HailValue::Boolean(flag)))
}

mod helpers {
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
    pub(crate) fn presence_array(i: &[u8], field_count: usize) -> IResult<&[u8], Vec<bool>> {
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

        Ok((rest, are_present))
    }
}
