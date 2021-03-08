use serde::de::Visitor;
use serde::Deserialize;

type Required = bool;
type NumberOfOptionalFields = usize;

/// This enum is at the core of the Look Ahead deserializer, as the deserializer runs it uses these
/// values to build a simplified schema of the Rust type being deserialized.
#[derive(Debug, Clone, PartialEq)]
pub enum StructureNode {
    /// Sequences (like a tuple or struct) for which the deserializer is told
    /// the length. The length is usually known at compile-time, but in exceptional cases
    /// (like [NDArray](crate::types::NDArray)) it might be provided dynamically.
    GivenLengthSequence(NumberOfOptionalFields, Vec<(Required, StructureNode)>),
    /// Sequences with unknown length, the deserializer will have to get it from the data.
    /// These sequences are assumed to be homogeneous by [LookAheadDeserializer].
    VariableLengthSequence(Box<(Required, StructureNode)>),
    /// Maps also usually have to determine their own length, but the key and value types are
    /// homogeneous.
    Map {
        key: Box<(Required, StructureNode)>,
        value: Box<(Required, StructureNode)>,
    },
    /// A Leaf is anything that does not contain another type.
    /// Of note is the fact that a NewType Struct is considered to be transparent to the type it
    /// contains, so it's not gonna result in a Leaf (or anything else).
    Leaf,
}

/// This enum is used to determine whether a field will be an option.
/// A sequence will push [OptionStatus::NextMayBeOption] to a stack, and the option, if present,
/// will turn it into a [OptionStatus::WasOption].
/// Note: an OptionStatus must be pushed to the stack before every field, with the exception of
/// the root, to avoid interference.
enum OptionStatus {
    NextMayBeOption,
    WasOption,
}

/// The LookAheadDeserializer parses and arbitrary type into a fictitious value that may be
/// discarded. Of interest is however its structure field which will have been filled out during
/// deserialization. That field (which was initialized as an empty vector) should contain a single
/// remaining value representing the analysis of the Rust type.
#[derive(Default)]
struct LookAheadDeserializer {
    structure: Vec<StructureNode>,
    option_statuses: Vec<OptionStatus>,
}

/// This helper function analyzes its generic type paramenter and returns a [StructureNode]
/// representing the shape of the type.
pub(crate) fn from_type<'a, T: Deserialize<'a>>() -> Result<StructureNode> {
    let mut deserializer = LookAheadDeserializer::default();
    let _ = T::deserialize(&mut deserializer)?;
    deserializer.structure.pop().ok_or(Error::NoFinalValue)
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut LookAheadDeserializer {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }
    serde::forward_to_deserialize_any! { enum identifier ignored_any }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_bool(false)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_i8(1)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_i16(1)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_i32(1)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_i64(1)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_u8(1)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_u16(1)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_u32(1)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_u64(1)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_f32(1.)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_f64(1.)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_char('1')
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_str("1")
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_string("1".to_owned())
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_bytes(&[1])
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_byte_buf(vec![1])
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.option_statuses.pop() {
            Some(OptionStatus::NextMayBeOption) => {
                self.option_statuses.push(OptionStatus::WasOption)
            }
            Some(OptionStatus::WasOption) => return Err(Error::UnexpectedWasOptionFound),
            None => return Err(Error::NoOptionStatus),
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.structure.push(StructureNode::Leaf);
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        // Newtype structs are usually considered to be 'invisible' wrappers around their inner
        // value for deserialization purposes, so no Leaf here.
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(VariableLengthSequenceAccess {
            de: &mut self,
            done: false,
        })
    }

    fn deserialize_tuple<V: Visitor<'de>>(mut self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(GivenLengthSequenceAccess {
            de: &mut self,
            len,
            fields: vec![],
        })
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

    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(MapSequenceAccess {
            de: &mut self,
            key: None,
        })
    }
}

/// This struct handles any sequence of variable length (essentially calls to
/// `deserializer.deserialize_seq`).
/// For the LookAheadDeserializer variable length sequences are assumed to be homogeneous and only
/// a single element will be deserialized.
struct VariableLengthSequenceAccess<'a> {
    de: &'a mut LookAheadDeserializer,
    done: bool,
}
impl<'de, 'a> serde::de::SeqAccess<'de> for VariableLengthSequenceAccess<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.done {
            return Ok(None);
        }

        self.de.option_statuses.push(OptionStatus::NextMayBeOption);

        let result = seed.deserialize(&mut *self.de)?;

        {
            // Get the element node that should have just been pushed during deserialization
            let element_node = (self.de.structure)
                .pop()
                .ok_or(Error::UnexpectedlyExaustedStructureNodes)?;

            // Check the OptionStatus
            let element_is_required = match self.de.option_statuses.pop() {
                Some(OptionStatus::NextMayBeOption) => true, // Required
                Some(OptionStatus::WasOption) => false,      // Not required
                None => return Err(Error::NoOptionFlag),
            };

            // Build the full senquence_node, homogeneous sequences have a single type.
            let sequence_node = StructureNode::VariableLengthSequence(Box::new((
                element_is_required,
                element_node,
            )));

            self.de.structure.push(sequence_node);
        }

        self.done = true;

        Ok(Some(result))
    }
}

/// This struct handles any sequence of given length (for example tuples and structs).
///
/// Of note is the fact that, given the custom [Deserialize](serde::Deserialize) implementation
/// of [NDArray](crate::types::NDArray), it is not impossible for homogeneous sequences to be
/// parsed this way. In the case of the NDArray this results in a length of 1 being read (all
/// numbers return 1) and thus a single field being recorded. This must be handled appropriately
/// in the actual Hail Deserializer.
struct GivenLengthSequenceAccess<'a> {
    de: &'a mut LookAheadDeserializer,
    len: usize,
    fields: Vec<(bool, StructureNode)>,
}
impl<'de, 'a> serde::de::SeqAccess<'de> for GivenLengthSequenceAccess<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        self.de.option_statuses.push(OptionStatus::NextMayBeOption);

        let result = seed.deserialize(&mut *self.de)?;

        {
            // Get the field node that should have just been pushed during deserialization
            let field_node = (self.de.structure)
                .pop()
                .ok_or(Error::UnexpectedlyExaustedStructureNodes)?;

            // Check the OptionStatus
            let field_node_is_required = match self.de.option_statuses.pop() {
                Some(OptionStatus::NextMayBeOption) => true, // Required
                Some(OptionStatus::WasOption) => false,      // Not required
                None => return Err(Error::NoOptionFlag),
            };

            // Store it for later, we need all the fields before constructing the sequence_node.
            self.fields.push((field_node_is_required, field_node));
        }

        self.len -= 1;

        // When we have processed the last field, construct and push the sequence_node.
        if self.len == 0 {
            let number_of_options = self.fields.iter().filter(|(required, _)| !required).count();

            let sequence_node =
                StructureNode::GivenLengthSequence(number_of_options, self.fields.clone());

            self.de.structure.push(sequence_node);
        };

        Ok(Some(result))
    }
}

/// This struct handles access to maps (like Dict in the case of Hail).
/// The keys and the values are assumed to be respectively homogeneous, and that the key
/// will be deserialized before the value.
struct MapSequenceAccess<'a> {
    de: &'a mut LookAheadDeserializer,
    key: Option<(bool, StructureNode)>,
}
impl<'de, 'a> serde::de::MapAccess<'de> for MapSequenceAccess<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.key.is_some() {
            return Ok(None);
        }

        self.de.option_statuses.push(OptionStatus::NextMayBeOption);

        let result = seed.deserialize(&mut *self.de)?;

        {
            // Get the key node that should have just been pushed during deserialization
            let key_node = (self.de.structure)
                .pop()
                .ok_or(Error::UnexpectedlyExaustedStructureNodes)?;

            // Check the OptionStatus
            let key_is_required = match self.de.option_statuses.pop() {
                Some(OptionStatus::NextMayBeOption) => true, // Required
                Some(OptionStatus::WasOption) => false,      // Not required
                None => return Err(Error::NoOptionFlag),
            };

            // Store it for later, we need both the key and value nodes to build the map_node
            self.key = Some((key_is_required, key_node))
        }

        Ok(Some(result))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.de.option_statuses.push(OptionStatus::NextMayBeOption);

        let result = seed.deserialize(&mut *self.de)?;

        {
            // Get the value node that should have just been pushed during deserialization
            let value_node = (self.de.structure)
                .pop()
                .ok_or(Error::UnexpectedlyExaustedStructureNodes)?;

            // Check the OptionStatus
            let value_is_required = match self.de.option_statuses.pop() {
                Some(OptionStatus::NextMayBeOption) => true, // Required
                Some(OptionStatus::WasOption) => false,      // Not required
                None => return Err(Error::NoOptionFlag),
            };

            let value = (value_is_required, value_node);

            // At this point we should already have the key.
            if let Some(key) = &self.key {
                let map_node = StructureNode::Map {
                    key: Box::new(key.clone()),
                    value: Box::new(value),
                };
                self.de.structure.push(map_node);
            } else {
                return Err(Error::MapValueBeforeKey);
            }
        }

        Ok(result)
    }
}

use error::{Error, Result};

mod error {
    use std::fmt;
    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        Custom(String),

        /// Once a type has been deserialized, the a StructureNode is supposed to be present in the
        /// deserializer. This error is raised if that's not the case.
        NoFinalValue,

        /// This error is raised if during the deserialization of an Option type the deserializer
        /// does not have an [OptionStatus](super::OptionStatus). An OptionStatus is always supposed
        /// to be pushed to the `option_statuses` stack of the deserializer before any new field
        /// which might be an Option.
        NoOptionStatus,
        /// [OptionStatus::WasOption](super::OptionStatus) is what an Option uses to signal the
        /// parent sequence that the deserialized field is optional. If an Option find this value
        /// during deserialization it means that something has gone wrong.
        UnexpectedWasOptionFound,

        /// Every time an element in a sequence is deserialized, a corresponding element should have
        /// been pushed to the stack in the deserializer. This error is raised if that stack is
        /// found to be unexpectedly empty.
        UnexpectedlyExaustedStructureNodes,
        /// Similarly to [Error::UnexpectedlyExaustedStructureNodes], after deserializing
        /// a sequence element we expect an option status to be present in the stack.
        NoOptionFlag,

        /// The Map handling assumes that keys are always deserialized before values, so if no
        /// corresponding key is found after deserializing the value this error will be raised.
        MapValueBeforeKey,
    }

    impl serde::ser::Error for Error {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            Error::Custom(msg.to_string())
        }
    }

    impl serde::de::Error for Error {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            Error::Custom(msg.to_string())
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            use Error::*;
            let message = match self {
                Custom(msg) => msg,

                NoFinalValue => "the final structure array is empty",

                NoOptionStatus => "no OptionStatus available during Option deserialization",
                UnexpectedWasOptionFound => "WasOption found during Option deserialization",

                UnexpectedlyExaustedStructureNodes => {
                    "no StructureNode is left after deserializing an element/field of a sequence"
                }
                NoOptionFlag => "no OptionStatus flag available during sequence deserialization",

                MapValueBeforeKey => "no key structure is available after value deserialization",
            };
            formatter.write_str(message)
        }
    }

    impl std::error::Error for Error {}

    impl From<Error> for crate::Error {
        fn from(e: Error) -> Self {
            crate::Error::LookAheadError(e.to_string())
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

mod tests {
    #![allow(unused_imports)]
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use super::{from_type, StructureNode};

    #[test]
    fn test_bool() {
        assert_eq!(StructureNode::Leaf, from_type::<bool>().unwrap());
    }

    #[test]
    fn test_seq_with_required_elements() {
        assert_eq!(
            StructureNode::VariableLengthSequence(Box::new((true, StructureNode::Leaf))),
            from_type::<Vec<bool>>().unwrap()
        );
    }

    #[test]
    fn test_seq_with_optional_elements() {
        assert_eq!(
            StructureNode::VariableLengthSequence(Box::new((false, StructureNode::Leaf))),
            from_type::<Vec<Option<bool>>>().unwrap()
        );
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct TestTuple(bool, Option<bool>);

        assert_eq!(
            StructureNode::GivenLengthSequence(
                1,
                vec!((true, StructureNode::Leaf), (false, StructureNode::Leaf))
            ),
            from_type::<TestTuple>().unwrap()
        );

        #[derive(Debug, PartialEq, Deserialize)]
        struct ParentTestTuple(Option<TestTuple>, TestTuple);

        assert_eq!(
            StructureNode::GivenLengthSequence(
                1,
                vec![
                    (
                        false,
                        StructureNode::GivenLengthSequence(
                            1,
                            vec!((true, StructureNode::Leaf), (false, StructureNode::Leaf))
                        )
                    ),
                    (
                        true,
                        StructureNode::GivenLengthSequence(
                            1,
                            vec!((true, StructureNode::Leaf), (false, StructureNode::Leaf))
                        )
                    )
                ]
            ),
            from_type::<ParentTestTuple>().unwrap()
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
            StructureNode::GivenLengthSequence(
                2,
                vec![
                    (
                        true,
                        StructureNode::GivenLengthSequence(
                            1,
                            vec!(
                                (true, StructureNode::Leaf),
                                (false, StructureNode::Leaf),
                                (
                                    true,
                                    StructureNode::GivenLengthSequence(
                                        1,
                                        vec![
                                            (true, StructureNode::Leaf),
                                            (false, StructureNode::Leaf),
                                        ]
                                    )
                                ),
                            )
                        )
                    ),
                    (
                        false,
                        StructureNode::GivenLengthSequence(
                            1,
                            vec!(
                                (true, StructureNode::Leaf),
                                (false, StructureNode::Leaf),
                                (
                                    true,
                                    StructureNode::GivenLengthSequence(
                                        1,
                                        vec![
                                            (true, StructureNode::Leaf),
                                            (false, StructureNode::Leaf),
                                        ]
                                    )
                                ),
                            )
                        )
                    ),
                    (false, StructureNode::Leaf)
                ]
            ),
            from_type::<ParentStruct>().unwrap()
        );
    }

    #[test]
    fn test_map() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Test(BTreeMap<bool, bool>);
        assert_eq!(
            StructureNode::Map {
                key: Box::new((true, StructureNode::Leaf)),
                value: Box::new((true, StructureNode::Leaf)),
            },
            from_type::<Test>().unwrap()
        );

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestTwo(BTreeMap<bool, Option<bool>>);
        assert_eq!(
            StructureNode::Map {
                key: Box::new((true, StructureNode::Leaf)),
                value: Box::new((false, StructureNode::Leaf)),
            },
            from_type::<TestTwo>().unwrap()
        );

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestThree(BTreeMap<Option<bool>, Option<bool>>);
        assert_eq!(
            StructureNode::Map {
                key: Box::new((false, StructureNode::Leaf)),
                value: Box::new((false, StructureNode::Leaf)),
            },
            from_type::<TestThree>().unwrap()
        );

        #[derive(Debug, PartialEq, Deserialize)]
        struct TestFour(BTreeMap<Option<bool>, bool>);
        assert_eq!(
            StructureNode::Map {
                key: Box::new((false, StructureNode::Leaf)),
                value: Box::new((true, StructureNode::Leaf)),
            },
            from_type::<TestFour>().unwrap()
        );
    }
}
