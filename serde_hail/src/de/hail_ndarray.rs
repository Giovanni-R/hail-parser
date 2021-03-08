use std::{convert::TryInto, marker::PhantomData};

use serde::{
    de::{DeserializeOwned, DeserializeSeed, Error, SeqAccess, Visitor},
    Deserialize,
};

use crate::types::NDArray;

/// A NDArray needs a custom deserialization method because it is a variable-length sequence which
/// does not fit in the same structure as other Hail variable-length sequences.
///
/// This is because the Hail deserializer assumes that variable length sequences can be
/// parsed by first reading their length and then deserializing each element one by one.
/// This is not true for a NDArray which has a 'composite' length (a value for each axis).
///
/// This custom deserialization works by first deserializing a sequence of two, which is handled by
/// NDArrayVisitor, then combining the data into a NDArray.
impl<'de, T: 'de + DeserializeOwned, const N: usize> Deserialize<'de> for NDArray<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the data
        let (dims, data) = deserializer.deserialize_tuple(2, NDArrayVisitor::<T, N>::new())?;

        // Put the data into a ndarray
        {
            // This is safe, but will cause loss of data if any of the dimensions are more than
            // the native usize.
            let size_dims: Vec<usize> = dims.iter().map(|d| *d as usize).collect();

            let shape = {
                // Trait brings .f() into scope.
                use ndarray::ShapeBuilder;

                // Here we tell ndarray the shape of our data.
                // Note that by default ndarray uses a c-order (row-major) while Hail uses
                // a f-order (column-major), so we need to set it.
                //                      ↓↓↓ This is what we want.
                // row-major(c)    column-major(f)
                // [[1, 2, 3],     [[1, 4, 7],
                //  [4, 5, 6],      [2, 5, 8],
                //  [7, 8, 9]]      [3, 6, 9]]
                ndarray::IxDyn(size_dims.as_slice()).f()
            };

            match ndarray::ArrayD::from_shape_vec(shape, data) {
                Ok(array) => Ok(NDArray(array)),
                Err(_) => Err(Error::custom("unable parse ndarray")),
            }
        }
    }
}

/// This visitor handles two elements:
/// - A sequence of length N, where N is the number of dimensions of the array.
///   These values are the size of the array axes.
/// - A sequence of variable length (the number of actual) of the inner type of the NDArray.
///   The length is determined by the total size and known only after deserializing the sequence
///   above.
///
/// Both sequences make use of [SequenceWithLengthSeed] which implements [DeserializeSeed] instead
/// of Deserialize to allow a variable-length sequence to be 'masqueraded' as a given-length one.
///
/// Note that N is known at compile-time, so the fist sequence could be deserialized using
/// const-generic arrays once they have been implemented in serde.
struct NDArrayVisitor<T, const N: usize>(PhantomData<T>);
impl<T, const N: usize> NDArrayVisitor<T, N> {
    fn new() -> Self {
        Self(PhantomData)
    }
}
impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for NDArrayVisitor<T, N> {
    type Value = (Vec<i64>, Vec<T>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "ndarray")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let dimensions: Vec<i64> = seq
            .next_element_seed(SequenceWithLengthSeed::<i64>::new(N))?
            .ok_or_else(|| Error::custom("expected a sequence of ndarray dimensions"))?;

        // Compute the total number of elements.
        let number_of_elements: usize = {
            // Overflow will cause a panic in debug mode, it's memory-safe but incorrect
            // in release mode.
            let n: i64 = dimensions.iter().product();

            n.try_into()
                .map_err(|_| Error::custom(&format!("unable to convert value to usize ({})", n)))?
        };

        let data: Vec<T> = seq
            .next_element_seed(SequenceWithLengthSeed::<T>::new(number_of_elements))?
            .ok_or_else(|| Error::custom("expected a valid value"))?;

        Ok((dimensions, data))
    }
}

/// SequenceWithLengthSeed implements [DeserializeSeed] instead of [Deserialize] so that it might
/// access its own state during deserialization (because the method is called on an instance,
/// not 'on the type').
/// This allows [NDArrayVisitor] to call `next_element_seed` instead of `next_element` and pass
/// along a value that holds the length of the sequence it needs.
/// This is useful because there isn't a type (which would be a compile-time construct) that by
/// itself can include a fixed length that can only be determined at run-time.
struct SequenceWithLengthSeed<T> {
    len: usize,
    _phantom: PhantomData<T>,
}
impl<'de, T: Deserialize<'de>> SequenceWithLengthSeed<T> {
    fn new(len: usize) -> Self {
        Self {
            len,
            _phantom: PhantomData,
        }
    }
}
impl<'de, T: Deserialize<'de>> DeserializeSeed<'de> for SequenceWithLengthSeed<T> {
    /// The type produced by using this seed.
    type Value = Vec<T>;

    /// Equivalent to the more common `Deserialize::deserialize` method, except
    /// with some initial piece of data (the seed) passed in.
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InnerVisitor<T>(usize, PhantomData<T>);
        impl<'de, T: Deserialize<'de>> Visitor<'de> for InnerVisitor<T> {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an uniform sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut data = Vec::with_capacity(self.0);
                for _ in 0..self.0 {
                    let value: T = seq
                        .next_element()?
                        .ok_or_else(|| Error::custom("expected a valid value"))?;
                    data.push(value);
                }
                Ok(data)
            }
        }

        deserializer.deserialize_tuple(self.len, InnerVisitor(self.len, PhantomData))
    }
}
