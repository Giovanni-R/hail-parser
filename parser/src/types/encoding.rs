#[allow(unused_imports)]
use crate::HailValue; // For the docs

/// The Etype, when filled with the virtual hints, is everything that is needed to keep track
/// of a schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EType {
    /// The shape of an [EType] represents what the underlying structure actually is by combining
    /// recursively a limited set of primitives.
    pub shape: ETypeShape,
    pub required: bool,
    /// The [VirtualHint] allows the [EType] to be the single reference during parsing instead of
    /// juggling both the [EType] and [VType].
    /// `None` implies that that the default [HailValue] is the correct
    /// interpretation for a [EType] (a [ETypeShape::BaseStruct] to a
    /// [HailValue::Struct], for example).
    pub virtual_hint: Option<VirtualHint>,
}

/// The [ETypeShape] represents the way data is organised in the file, with arbitrary nesting of the
/// available variants.
///
/// Note: The physical encoding primitives (for example, how a u32 is physically encoded) are
/// defined in the [Encoding](crate::parse::Encoding) trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ETypeShape {
    /// A Struct is simply a list of named fields, where the fields will appear in order during
    /// parsing.
    BaseStruct(Vec<(String, EType)>),
    Array(Box<EType>),
    NdArrayColumnMajor(Box<EType>, u32),
    Binary,
    Float32,
    Float64,
    /// All physical representations for this are *unsigned* (either u32le or +LEB128).
    Int32,
    Int64,
    Boolean,
}

/// A [VirtualHint] is a way to tell the parser how to interpret a specific [EType] into a
/// [HailValue]. Only cases where the interpretation would differ from the
/// default are present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VirtualHint {
    /// [ETypeShape::Array] → [HailValue::Set]
    Set,
    /// [ETypeShape::Array] of [ETypeShape::BaseStruct] with "key" and "value" fields
    /// → [HailValue::Dict]
    Dict,
    /// [ETypeShape::BaseStruct] with fields "includesStart", "start", "end", and "includesEnd"
    /// → [HailValue::Interval]
    Interval,
    /// [ETypeShape::BaseStruct] → [HailValue::Tuple]
    Tuple,
    /// [ETypeShape::Binary] → [HailValue::String]
    /// Currently this is the only use of the Binary variant.
    String,
    /// [ETypeShape::BaseStruct] with fields "contig" and "position" → [HailValue::Locus]
    /// The inner string is the reference genome.
    Locus(String),
    /// [ETypeShape::Int32] → [HailValue::Call]
    Call,
}

/// VType represents the virtual schema of a matrix table, table, or component.
/// It is always present and in older components is the only schema present.
/// Each [VTypeShape] has a corresponding default [ETypeShape], but in later components both are
/// present to allow a virtual type to have multiple encoded representations as appropriate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VType {
    /// The shape of a [VType] represents the intended conceptual rapresentation of a datapoint.
    pub shape: VTypeShape,
    pub required: bool,
}

/// The VTypeShape closely mirrors the variants of a [HailValue], with the exception of the
/// absence of a corresponding variant to [HailValue::Missing].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VTypeShape {
    Struct(Vec<(String, VType)>),
    Tuple(Vec<VType>),

    Array(Box<VType>),
    Set(Box<VType>),
    Dict(Box<VType>, Box<VType>),
    NDArray(Box<VType>, u32),
    Interval(Box<VType>),

    String,

    Float32,
    Float64,
    Int32,
    Int64,

    Boolean,

    Locus(String),
    Call,
}
