use serde::de::{MapAccess, Visitor};
use serde::Deserialize;
use std::convert::TryInto;

use parser::parse;
use parser::parse::Encoding;

use crate::{Error, Result};

use super::{look_ahead, look_ahead::StructureNode};

/// Optional fields are marked as present or absent at the beginning of a sequence,
/// the sequence communicates whether a specific field is present by pushing one of the values
/// of this enum to a stack.
#[derive(Debug, Clone)]
enum OptionFlag {
    NextIsSome,
    NextIsNone,
}

/// The deserializer uses the structure to know the location of optional fields, the
/// option_statuses to keep track of whether the upcoming optional fields are present ot not,
/// and the generic [Encoding] to interpret the physical layout of the data correctly.
#[derive(Debug)]
pub struct Deserializer<'de, 's, E: Encoding> {
    input: &'de [u8],
    /// The structure is a primitive schema that keeps track of optional fields.
    /// It reflects the value currently being deserialized, but is restored to the top-level field
    /// when the deserializer is done.
    structure: &'s StructureNode,
    /// Option flags are pushed to this stack and popped in order by the deserialize_option method.
    option_flags: Vec<OptionFlag>,
    /// The [Encoding] allows the deserializer to be generic over the physical encoding of the
    /// primitive types.
    encoding: std::marker::PhantomData<E>,
}

impl<'de, 's, E: Encoding> Deserializer<'de, 's, E> {
    fn from_bytes(input: &'de [u8], structure: &'s StructureNode) -> Self {
        Deserializer {
            input,
            structure,
            option_flags: vec![],
            encoding: std::marker::PhantomData,
        }
    }
}

/// This is the main public interface of the module.
pub fn parse_rows<'de, T, E>(i: &'de [u8]) -> Result<Vec<T>>
where
    E: Encoding,
    T: Deserialize<'de>,
{
    let mut result = Vec::new();
    let structure = look_ahead::from_type::<T>()?;
    let mut deserializer: Deserializer<E> = Deserializer::from_bytes(i, &structure);
    while deserializer.parse_bool()? {
        let t: T = T::deserialize(&mut deserializer)?;
        result.push(t);
    }
    Ok(result)
}

impl<'de, 's, E: Encoding> Deserializer<'de, 's, E> {
    /// A helper function to unpack a nom Result and update the input value in the deserializer.
    fn update_and_return<T>(&mut self, parsed: nom::IResult<&'de [u8], T>) -> Result<T> {
        let (rest, value) = parsed?;
        self.input = rest;
        Ok(value)
    }

    /// This helper function reads the appropriate number of flags that describe the presence of
    /// optional fields, then pushes the result to the `option_flags` stack in the deserializer.
    fn load_options_flags(&mut self, number_of_options: usize) -> Result<()> {
        // First get all the option flags
        let presence_array = self.update_and_return(parse::data::helpers::presence_array(
            self.input,
            number_of_options,
        ))?;

        // For each value, record in the deserializer whether each option will be Some or None.
        // They are added in reverse because they will then be popped.
        for is_present in presence_array.into_iter().rev() {
            self.option_flags.push(match is_present {
                true => OptionFlag::NextIsSome,
                false => OptionFlag::NextIsNone,
            })
        }

        Ok(())
    }

    fn get_length(&mut self) -> Result<usize> {
        let len = self.update_and_return(E::u32(self.input))?;

        // Note: a u32 should always fit into a usize as long as it's on a 32/64 bits system.
        len.try_into().map_err(|_| Error::InvalidLength(len))
    }

    fn parse_bool(&mut self) -> Result<bool> {
        self.update_and_return(E::bool(self.input))
    }
}

impl<'de, 'a, 't, E: Encoding> serde::de::Deserializer<'de> for &'a mut Deserializer<'de, 't, E> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType)
    }

    // These types are not currently supported.
    serde::forward_to_deserialize_any! {
        i8 i16 i128 u16 u128 char
        // str bytes
        enum identifier ignored_any
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bool = self.update_and_return(E::bool(self.input))?;
        visitor.visit_bool(bool)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let int = self.update_and_return(E::i32(self.input))?;
        visitor.visit_i32(int)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let int = self.update_and_return(E::i64(self.input))?;
        visitor.visit_i64(int)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let int = self.update_and_return(E::u8(self.input))?;
        visitor.visit_u8(int)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let int = self.update_and_return(E::u32(self.input))?;
        visitor.visit_u32(int)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let int = self.update_and_return(E::u64(self.input))?;
        visitor.visit_u64(int)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let float = self.update_and_return(E::f32(self.input))?;
        visitor.visit_f32(float)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let float = self.update_and_return(E::f64(self.input))?;
        visitor.visit_f64(float)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let s = self.update_and_return(E::string(self.input))?;
        visitor.visit_string(s)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let s = self.update_and_return(E::str(self.input))?;
        visitor.visit_str(s)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bytes = self.update_and_return(E::byte_buf(self.input))?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bytes = self.update_and_return(E::bytes(self.input))?;
        visitor.visit_bytes(bytes)
    }

    // This function relies on global Deserializer state to know whether an option is present.
    // It is assumed that the option's container sets self.option_flags in the correct order before
    // this is called.
    // It is also assumed that the top-level container or value is not optional.
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let Some(status) = self.option_flags.pop() {
            match status {
                OptionFlag::NextIsSome => visitor.visit_some(self),
                OptionFlag::NextIsNone => visitor.visit_none(),
            }
        } else {
            Err(Error::NoOptionFlag(self.structure.clone()))
        }
    }

    // We need to choose between newtype struct transparency (`struct Call(u32)` ~ `u32`)
    // and correctly handling something like `struct Tuple(Option<i32>)` which requires checking
    // the presence flags of the tuple (which is a struct for Hail).
    // Newtype transparency is preferred because:
    // - This is the standard in Serde and Rust code.
    // - A tuple can be represented as actual tuple (vs a tuple-struct), which would sidestep this
    //   problem.
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value> {
        let len = self.get_length()?;

        let sequence_structure = self.structure;

        if let StructureNode::VariableLengthSequence(element) = sequence_structure {
            visitor.visit_seq(HomogeneousSequenceAccess::new(
                &mut self,
                sequence_structure,
                element,
                len,
            )?)
        } else {
            Err(Error::ExpectedVariableLengthSequence(
                sequence_structure.clone(),
            ))
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(mut self, len: usize, visitor: V) -> Result<V::Value> {
        let sequence_structure = self.structure;

        if let StructureNode::GivenLengthSequence(optional_count, field_structures) =
            sequence_structure
        {
            // Here there are two options:
            // - field_structures.len() == len
            //   The sequence is a normal tuple or struct, with a pre-defined number of fields.
            // - field_structures.len() == 1 && len > 1
            //   The sequence has only one structure element, but the length is indicated to be
            //   more than 1.
            //   This indicates that we are deserializing a NDArray which is modelled as a sequence
            //   of variable-length from the Deserialize/Visitor point of view, and given-length
            //   from the Deserializer point of view.
            // Also note that an NDArray of length 1 can be successfully deserialized using both
            // methods so no need to separate the case.

            if field_structures.len() == len {
                self.load_options_flags(*optional_count)?;

                visitor.visit_seq(HeterogeneousSequenceAccess {
                    de: &mut self,
                    sequence_structure,
                    field_structures,
                    len,
                })
            } else if field_structures.len() == 1 && len > 1 {
                visitor.visit_seq(HomogeneousSequenceAccess::new(
                    &mut self,
                    sequence_structure,
                    &field_structures[0],
                    len,
                )?)
            } else {
                Err(Error::UnexpectedGivenLengthSequenceLayout(
                    sequence_structure.clone(),
                    len,
                ))
            }
        } else {
            Err(Error::ExpectedGivenLengthSequence(
                sequence_structure.clone(),
                len,
            ))
        }
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.get_length()?;

        visitor.visit_map(MapSequenceAccess::new(&mut self, len)?)
    }
}

use serde::de::{DeserializeSeed, SeqAccess};

struct HomogeneousSequenceAccess<'a, 'de: 'a, 's, E: Encoding> {
    de: &'a mut Deserializer<'de, 's, E>,
    sequence_structure: &'s StructureNode,
    len: usize,
}
impl<'a, 'de, 's, E: Encoding> HomogeneousSequenceAccess<'a, 'de, 's, E> {
    fn new(
        de: &'a mut Deserializer<'de, 's, E>,
        sequence_structure: &'s StructureNode,
        element: &'s (bool, StructureNode),
        len: usize,
    ) -> Result<Self> {
        let (required, element_structure) = element;

        // Update the structure now that we are deserializing an element.
        // HomogeneousSequenceAccess will take care of putting the sequence_structure back when it's
        // done.
        de.structure = element_structure;

        if !required {
            de.load_options_flags(len)?;
        }

        Ok(HomogeneousSequenceAccess {
            de,
            sequence_structure,
            len,
        })
    }
}
impl<'a, 'de, 'et, E: Encoding> SeqAccess<'de> for HomogeneousSequenceAccess<'a, 'de, 'et, E> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        let result = seed.deserialize(&mut *self.de)?;

        self.len -= 1;

        // Restore the sequence structure at the end of the last cycle.
        if self.len == 0 {
            self.de.structure = self.sequence_structure
        }

        Ok(Some(result))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct HeterogeneousSequenceAccess<'a, 'de: 'a, 'et, E: Encoding> {
    de: &'a mut Deserializer<'de, 'et, E>,
    sequence_structure: &'et StructureNode,
    field_structures: &'et Vec<(bool, StructureNode)>,
    len: usize,
}
impl<'a, 'de, 'et, E: Encoding> SeqAccess<'de> for HeterogeneousSequenceAccess<'a, 'de, 'et, E> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        // Set the structure to the upcoming element
        self.de.structure = {
            let field_number = self.field_structures.len() - self.len;
            let (_, field_node) = &self.field_structures[field_number];
            field_node
        };

        let result = seed.deserialize(&mut *self.de)?;

        self.len -= 1;

        // Restore the sequence structure at the end of the last cycle.
        if self.len == 0 {
            self.de.structure = self.sequence_structure
        }

        Ok(Some(result))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapSequenceAccess<'a, 'de, 'et, E: Encoding> {
    de: &'a mut Deserializer<'de, 'et, E>,
    sequence_structure: &'et StructureNode,
    key_structure: &'et StructureNode,
    value_structure: &'et StructureNode,
    number_of_options: usize,
    len: usize,
}
impl<'a, 'de, 'et, E: Encoding> MapSequenceAccess<'a, 'de, 'et, E> {
    fn new(de: &'a mut Deserializer<'de, 'et, E>, len: usize) -> Result<Self> {
        let sequence_structure = de.structure;

        if let StructureNode::Map { key, value } = sequence_structure {
            let (key_required, key_structure) = key.as_ref();
            let (value_required, value_structure) = value.as_ref();

            // The keys and values are not marked as required in the encoded types.
            let number_of_options = (!key_required) as usize + (!value_required) as usize;

            Ok(Self {
                de,
                sequence_structure,
                key_structure,
                value_structure,
                number_of_options,
                len,
            })
        } else {
            Err(Error::ExpectedMap(sequence_structure.clone()))
        }
    }
}
impl<'a, 'de, 'et, E: Encoding> MapAccess<'de> for MapSequenceAccess<'a, 'de, 'et, E> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        // Before each key-value pair, we need to parse the option flags present because they are
        // modelled as a struct (struct { key: KeyType, value: ValueType }).
        self.de.load_options_flags(self.number_of_options)?;

        // Set the structure to the upcoming element
        self.de.structure = &self.key_structure;

        let result = seed.deserialize(&mut *self.de)?;

        // Note that we don't decrease by one, that happens after the corresponding value is
        // deserialized.

        Ok(Some(result))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // Set the structure to the upcoming element
        self.de.structure = &self.value_structure;

        let result = seed.deserialize(&mut *self.de)?;

        self.len -= 1;

        // Restore the sequence structure at the end of the last cycle.
        if self.len == 0 {
            self.de.structure = self.sequence_structure
        }

        Ok(result)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

mod tests {
    #![allow(unused_imports)]
    use serde::Deserialize;

    use parser::parse::StandardEncoder;
    use parser::types::{EType, ETypeShape};

    use super::parse_rows;
    #[cfg(test)]
    const ROW: u8 = 1u8;
    #[cfg(test)]
    const NO_ROW: u8 = 0u8;

    #[test]
    fn test_bool() {
        assert_eq!(
            vec![false],
            parse_rows::<bool, StandardEncoder>(&[ROW, 0u8, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![true],
            parse_rows::<bool, StandardEncoder>(&[ROW, 1u8, NO_ROW]).unwrap()
        );
        let hopefully_an_error = parse_rows::<bool, StandardEncoder>(&[ROW, 2u8, NO_ROW]);
        if hopefully_an_error.is_ok() {
            panic!(
                "was expecting an error got {:?} instead.",
                hopefully_an_error
            )
        };
    }

    #[test]
    fn test_seq_with_required_elements() {
        type Type = Vec<bool>;

        assert_eq!(
            vec![Vec::<bool>::new()],
            parse_rows::<Type, StandardEncoder>(&[ROW, 0u8, 0, 0, 0, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![vec![false]],
            parse_rows::<Type, StandardEncoder>(&[ROW, 1u8, 0, 0, 0, 0, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![vec![false, true, false, true, true]],
            parse_rows::<Type, StandardEncoder>(&[ROW, 5u8, 0, 0, 0, 0, 1, 0, 1, 1, NO_ROW])
                .unwrap()
        );
    }

    #[test]
    fn test_seq_with_optional_elements() {
        type Type = Vec<Option<bool>>;
        assert_eq!(
            vec![Vec::<Option<bool>>::new()],
            parse_rows::<Type, StandardEncoder>(&[ROW, 0u8, 0, 0, 0, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![vec![Some(false)]],
            parse_rows::<Type, StandardEncoder>(&[ROW, 1u8, 0, 0, 0, 0b0000_0000, 0, NO_ROW])
                .unwrap()
        );
        assert_eq!(
            vec![vec![Some(false), None, Some(false), None, Some(true)]],
            parse_rows::<Type, StandardEncoder>(&[ROW, 5u8, 0, 0, 0, 0b0000_1010, 0, 0, 1, NO_ROW])
                .unwrap()
        );
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct TestTuple(bool, Option<bool>);

        assert_eq!(
            vec![TestTuple(false, Some(false))],
            parse_rows::<TestTuple, StandardEncoder>(&[ROW, 0u8, 0, 0, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![TestTuple(false, Some(true))],
            parse_rows::<TestTuple, StandardEncoder>(&[ROW, 0u8, 0, 1, NO_ROW]).unwrap()
        );
        assert_eq!(
            vec![TestTuple(false, None)],
            parse_rows::<TestTuple, StandardEncoder>(&[ROW, 1u8, 0, NO_ROW]).unwrap()
        );

        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(rename = "?!")]
        struct ParentTestTuple(Option<TestTuple>, TestTuple);

        assert_eq!(
            vec![ParentTestTuple(
                Some(TestTuple(false, Some(true))),
                TestTuple(false, Some(true))
            )],
            parse_rows::<ParentTestTuple, StandardEncoder>(&[
                ROW,
                0b0000_0000, // parent flags
                0b0000_0000, // child 1 flags
                0,
                1,
                0b0000_0000, // child 2 flags
                0,
                1,
                NO_ROW
            ])
            .unwrap()
        );
        assert_eq!(
            vec![ParentTestTuple(None, TestTuple(false, Some(true)))],
            parse_rows::<ParentTestTuple, StandardEncoder>(&[
                ROW,
                0b0000_0001, // parent flags
                0b0000_0000, // child 2 flags
                0,
                1,
                NO_ROW
            ])
            .unwrap()
        );
        assert_eq!(
            vec![ParentTestTuple(
                Some(TestTuple(false, None)),
                TestTuple(false, Some(true))
            )],
            parse_rows::<ParentTestTuple, StandardEncoder>(&[
                ROW,
                0b0000_0000, // parent flags
                0b0000_0001, // child 1 flags
                0,
                0b0000_0000, // child 2 flags
                0,
                1,
                NO_ROW
            ])
            .unwrap()
        );
        assert_eq!(
            vec![ParentTestTuple(
                Some(TestTuple(false, None)),
                TestTuple(false, None)
            )],
            parse_rows::<ParentTestTuple, StandardEncoder>(&[
                ROW,
                0b0000_0000, // parent flags
                0b0000_0001, // child 1 flags
                0,
                0b0000_0001, // child 2 flags
                0,
                NO_ROW
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Tuple(bool, Option<bool>);
        #[derive(Debug, PartialEq, Deserialize)]
        struct BaseStruct {
            a: bool,
            b: Option<bool>,
            c: Tuple,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct ParentStruct {
            f: BaseStruct,
            s: Option<BaseStruct>,
            t: Option<bool>,
        }
        assert_eq!(
            vec![ParentStruct {
                f: BaseStruct {
                    a: true,
                    b: Some(false),
                    c: Tuple(true, Some(false))
                },
                s: Some(BaseStruct {
                    a: true,
                    b: Some(false),
                    c: Tuple(true, Some(false))
                },),
                t: Some(true),
            }],
            parse_rows::<ParentStruct, StandardEncoder>(&[
                ROW,
                0b0000_0000, // parent start [flags]
                0b0000_0000, // child 1 start [flags]
                1,
                0,
                0b0000_0000, // Child 1.3 start [flags]
                1,
                0,
                0b0000_0000, // child 2 start [flags]
                1,
                0,
                0b0000_0000, // Child 2.3 start [flags]
                1,
                0,
                1, // child 3 start
                NO_ROW
            ])
            .unwrap()
        );
        assert_eq!(
            vec![ParentStruct {
                f: BaseStruct {
                    a: true,
                    b: Some(false),
                    c: Tuple(true, None)
                },
                s: Some(BaseStruct {
                    a: true,
                    b: None,
                    c: Tuple(true, None)
                },),
                t: None,
            }],
            parse_rows::<ParentStruct, StandardEncoder>(&[
                ROW,
                0b0000_0010, // parent start [flags]
                0b0000_0000, // child 1 start [flags]
                1,
                0,
                0b0000_0001, // Child 1.3 start [flags]
                1,
                0b0000_0001, // child 2 start [flags]
                1,
                0b0000_0001, // Child 2.3 start [flags]
                1,
                NO_ROW
            ])
            .unwrap()
        );
    }
}
