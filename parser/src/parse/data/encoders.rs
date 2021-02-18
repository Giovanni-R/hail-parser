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
    fn u32(i: &[u8]) -> IResult<&[u8], u32> {
        nom::number::complete::le_u32::<&[u8], nom::error::Error<&[u8]>>(i)
    }

    fn u64(i: &[u8]) -> IResult<&[u8], u64> {
        nom::number::complete::le_u64::<&[u8], nom::error::Error<&[u8]>>(i)
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

    fn string(i: &[u8]) -> IResult<&[u8], String> {
        let (rest, len) = Self::u32(i)?;

        let (rest, raw_string): (&[u8], &[u8]) = bytes::take(len)(rest)?;

        match std::str::from_utf8(raw_string) {
            Ok(s) => Ok((rest, s.to_owned())),
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
}

named!(take1, take!(1));
