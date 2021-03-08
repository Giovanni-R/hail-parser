use std::collections::BTreeMap;

use crate::types::metadata::{MatrixMetadata, TableMetadata};

use super::metadata::ComponentMetadata;

/// A Hail Matrix Table is a two dimensional-ish data structure which combines two lists
/// and a 2D table (plus globals):
/// - A list of rows where each row has a key and potentially some associated values.
/// - A list of columns, analogous to the list of rows.
/// - A matrix of entries, where single entries are identified by their row and column.
///
/// The usefulness of this data structure, as I understand it, lies in the fact that it allows
/// for storing information about two sets of entities separately (say, genomes and genome
/// locations), and also about each combination of the two (a specific genome at a specific
/// location).
#[derive(Clone, Debug)]
pub struct Matrix {
    pub globals: Component,
    pub cols: Component,
    pub rows: Component,
    pub entries: Component,
    pub metadata: MatrixMetadata,
}

/// A Hail table is essentially a list of key-value pairs (plus globals).
#[derive(Clone, Debug)]
pub struct Table {
    pub globals: Component,
    pub rows: Component,
    pub metadata: TableMetadata,
}

/// A Component is the backbone of Hail (matrix) tables, semantically, it's essentially a list of
/// key-value pairs.
/// On disk, it is a list of usually-compressed files and a metadata json file which holds the
/// schema.
#[derive(Clone, Debug)]
pub struct Component {
    pub data: Vec<Vec<HailValue>>,
    pub metadata: ComponentMetadata,
    // index: Option<Index>,
}

/// HailValue is an enum that represents any possible value value that data in Hail (matrix) tables
/// may take.
///
/// Of note for this implementation is the decision to introduce an additional invariant by
/// disallowing NaN values in floats to ensure ordering and hashing work correctly.
///
/// More is explained in the trait implementations and their module.
#[derive(Clone, Debug)]
pub enum HailValue {
    Struct(BTreeMap<String, HailValue>),
    Tuple(Vec<HailValue>),

    Array(Vec<HailValue>),
    Set(Vec<HailValue>),
    Dict(BTreeMap<HailValue, HailValue>),
    NDArray(ndarray::ArrayD<HailValue>),
    Interval {
        start: Box<HailValue>,
        end: Box<HailValue>,
        includes_start: bool,
        includes_end: bool,
    },

    String(String),

    Float32(f32),
    Float64(f64),
    Int32(u32),
    Int64(i64),

    Boolean(bool),

    Locus {
        contig: String,
        position: u32,
        reference: String,
    },
    Call(u32),
    Missing,
}

/// These traits are implemented manually because of the missing implementations on [f32] and [f64].
/// Those implementations are missing because the NaN value in floats is not equal to itself,
/// meaning that equality is not Reflexive, and unambiguous ordering is impossible as per the spec
/// (IEEE 754-2008 section 5.11).
///
/// This is solved by introducing a project level invariant where NaN values are replaced with
/// [HailValue::Missing] (the same value as for missing structs/array elements).
/// Order-wise, [HailValue::Missing] will always be the 'smallest' value and equal to itself.
///
/// More: <https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#how-can-i-implement-partialord>
mod hail_value_comparisons {
    use super::HailValue;

    /// [Eq] is a marker trait indicating that the [PartialEq] implementation is Reflexive
    /// (each value is equal to itself), which is not true for floats
    /// (as NaN is a valid value that is not equal to itself—or anything else for that matter).
    /// This means that NaN is *not* a supported value in [HailValue::Float32]/[HailValue::Float64].
    /// The parser will replace any NaN value with [HailValue::Missing].
    /// Note that [PartialEq] already guarantees the equality is Symmetric and Transitive.
    impl Eq for HailValue {}

    /// Ordering of instances of [HailValue] is possible because of two project-level invariants:
    /// - Any NaN value in floats is replaced by [HailValue::Missing] which is always the 'smallest'
    ///   number in any category.
    /// - There is no meaningless comparisons across categories. This mean that in this current
    ///   form we do not support mappings where the key can have different types.
    ///   Both [HailValue::Struct] and [HailValue::Dict] respect this invariant.
    ///   Note: if we wished to support that use-case in the future, we would need to
    ///    use a HashMap and carefully implement the [Hash](core::hash::Hash) trait to ensure it
    ///    remains aligned with [PartialEq] and [Ord]
    ///
    /// Given these invariants, any illegal ordering will be turned into a
    /// [Ordering::Less](std::cmp::Ordering::Less).
    impl Ord for HailValue {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
        }
    }

    /// Comparisons make sense only among instances of the same [HailValue] variant.
    /// Comparisons between the different numeric types are intentionally left out for now,
    /// but might be implemented in the future for convenience.
    impl PartialOrd for HailValue {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (HailValue::Struct(self_map), HailValue::Struct(other_map)) => {
                    self_map.partial_cmp(other_map)
                }
                (HailValue::Array(self_vec), HailValue::Array(other_vec)) => {
                    self_vec.partial_cmp(other_vec)
                }
                (HailValue::Set(self_vec), HailValue::Set(other_vec)) => {
                    self_vec.partial_cmp(other_vec)
                }
                (HailValue::Dict(self_map), HailValue::Dict(other_map)) => {
                    self_map.partial_cmp(other_map)
                }

                (HailValue::String(self_inner), HailValue::String(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }

                (HailValue::Float32(self_inner), HailValue::Float32(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }
                (HailValue::Float64(self_inner), HailValue::Float64(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }
                (HailValue::Int32(self_inner), HailValue::Int32(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }
                (HailValue::Int64(self_inner), HailValue::Int64(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }

                (HailValue::Boolean(self_inner), HailValue::Boolean(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }

                (
                    HailValue::Locus {
                        contig: self_contig,
                        position: self_position,
                        reference: self_reference,
                    },
                    HailValue::Locus {
                        contig: other_contig,
                        position: other_position,
                        reference: other_reference,
                    },
                ) => {
                    // Do contigs have an ordering?
                    if self_contig == other_contig && self_reference == other_reference {
                        self_position.partial_cmp(other_position)
                    } else {
                        None
                    }
                }
                (HailValue::Call(self_inner), HailValue::Call(other_inner)) => {
                    self_inner.partial_cmp(other_inner)
                }

                // A missing value is always the 'smallest' value.
                (HailValue::Missing, HailValue::Missing) => Some(std::cmp::Ordering::Equal),
                (HailValue::Missing, _) => Some(std::cmp::Ordering::Less),
                (_, HailValue::Missing) => Some(std::cmp::Ordering::Greater),

                // UNDEFINED BEHAVIOUR
                // If the two are not the same, then there is no point comparing.
                (_, _) => None,
            }
        }
    }

    /// In the current implementation each [HailValue] variant is equal only to instances of the
    /// same type when the contents are also the same.
    /// This means that direct equality between different numbers variants will always return false
    /// for now (but extended comparisons may be added later for convenience).
    ///
    /// [HailValue::Missing] is equal to itself and different from any other value.
    ///
    /// We are implementing [PartialEq] manually because it is not reccomended to implement some of
    /// these comparison trait manually and some by derivation.
    impl PartialEq for HailValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (HailValue::Struct(self_map), HailValue::Struct(other_map)) => {
                    self_map.eq(other_map)
                }
                (HailValue::Array(self_vec), HailValue::Array(other_vec)) => self_vec.eq(other_vec),
                (HailValue::Set(self_vec), HailValue::Set(other_vec)) => self_vec.eq(other_vec),
                (HailValue::Dict(self_map), HailValue::Dict(other_map)) => self_map.eq(other_map),

                (HailValue::String(self_inner), HailValue::String(other_inner)) => {
                    self_inner.eq(other_inner)
                }

                (HailValue::Float32(self_inner), HailValue::Float32(other_inner)) => {
                    self_inner.eq(other_inner)
                }
                (HailValue::Float64(self_inner), HailValue::Float64(other_inner)) => {
                    self_inner.eq(other_inner)
                }
                (HailValue::Int32(self_inner), HailValue::Int32(other_inner)) => {
                    self_inner.eq(other_inner)
                }
                (HailValue::Int64(self_inner), HailValue::Int64(other_inner)) => {
                    self_inner.eq(other_inner)
                }

                (HailValue::Boolean(self_inner), HailValue::Boolean(other_inner)) => {
                    self_inner.eq(other_inner)
                }

                (
                    HailValue::Locus {
                        contig: self_contig,
                        position: self_position,
                        reference: self_reference,
                    },
                    HailValue::Locus {
                        contig: other_contig,
                        position: other_position,
                        reference: other_reference,
                    },
                ) => {
                    self_contig.eq(other_contig)
                        && self_position.eq(other_position)
                        && self_reference.eq(other_reference)
                }
                (HailValue::Call(self_inner), HailValue::Call(other_inner)) => {
                    self_inner.eq(other_inner)
                }

                // A missing value is oly equal to itself
                (HailValue::Missing, HailValue::Missing) => true,
                (HailValue::Missing, _) => false,
                (_, HailValue::Missing) => false,

                // Currently, different HailValue variants are always different.
                (_, _) => false,
            }
        }
    }
}
