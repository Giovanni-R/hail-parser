use nom::bytes::complete as bytes;
use nom::{named, take, IResult};

/// This trait defines the physical encodying of the primitive data types
/// (numbers, booleans, and string).
///
/// The trait exists to allow for different encodings (like using LEB128 instead of u32) to be
/// monomorphised at compile time.
/// By default all numbers use their standard little endian encoding,
/// booleans are encoded as a byte (`0u8`/`1u8`), and strings as UTF-8 byte arrays with length
/// prepended (a u32).
pub trait Encoding {
    fn u8(i: &[u8]) -> IResult<&[u8], u8> {
        let (rest, value) = take1(i)?;
        Ok((rest, value[0]))
    }

    fn u32(i: &[u8]) -> IResult<&[u8], u32> {
        nom::number::complete::le_u32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn u64(i: &[u8]) -> IResult<&[u8], u64> {
        nom::number::complete::le_u64::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn i32(i: &[u8]) -> IResult<&[u8], i32> {
        nom::number::complete::le_i32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn i64(i: &[u8]) -> IResult<&[u8], i64> {
        nom::number::complete::le_i64::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn f32(i: &[u8]) -> IResult<&[u8], f32> {
        nom::number::complete::le_f32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn f64(i: &[u8]) -> IResult<&[u8], f64> {
        nom::number::complete::le_f64::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn bool(i: &[u8]) -> IResult<&[u8], bool> {
        match take1(i)? {
            (rest, [0u8]) => Ok((rest, false)),
            (rest, [1u8]) => Ok((rest, true)),
            (rest, _) => Err(nom::Err::Failure(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::IsNot,
            ))),
        }
    }

    fn bytes(i: &[u8]) -> IResult<&[u8], &[u8]> {
        let (rest, len) = Self::u32(i)?;

        bytes::take(len)(rest)
    }

    fn byte_buf(i: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (rest, bytes): (&[u8], &[u8]) = Self::bytes(i)?;

        Ok((rest, bytes.to_vec()))
    }

    fn str(i: &[u8]) -> IResult<&[u8], &str> {
        let (rest, raw_string): (&[u8], &[u8]) = Self::bytes(i)?;

        match std::str::from_utf8(raw_string) {
            Ok(s) => Ok((rest, s)),
            Err(_) => Err(nom::Err::Failure(nom::error::Error::new(
                i,
                nom::error::ErrorKind::ParseTo,
            ))),
        }
    }

    fn string(i: &[u8]) -> IResult<&[u8], String> {
        let (rest, raw_string): (&[u8], Vec<u8>) = Self::byte_buf(i)?;

        match String::from_utf8(raw_string) {
            Ok(s) => Ok((rest, s)),
            Err(_) => Err(nom::Err::Failure(nom::error::Error::new(
                i,
                nom::error::ErrorKind::ParseTo,
            ))),
        }
    }
}

pub struct StandardEncoder;
impl Encoding for StandardEncoder {}

pub struct UnsignedLEB128Encoder;
impl Encoding for UnsignedLEB128Encoder {
    fn u32(i: &[u8]) -> IResult<&[u8], u32> {
        nom_leb128::leb128_u32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn u64(i: &[u8]) -> IResult<&[u8], u64> {
        nom_leb128::leb128_u64::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn i32(i: &[u8]) -> IResult<&[u8], i32> {
        nom_leb128::leb128_i32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn i64(i: &[u8]) -> IResult<&[u8], i64> {
        nom_leb128::leb128_i64::<&[u8], nom::error::Error<&[u8]>>(i)
    }
}

named!(take1, take!(1));
