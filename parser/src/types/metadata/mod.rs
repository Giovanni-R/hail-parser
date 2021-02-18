use serde::Deserialize;

pub mod component;
pub mod component_1;
pub mod component_2;
pub mod matrix;
pub mod shared;
pub mod table;

pub use component::ComponentMetadata;
pub use matrix::MatrixMetadata;
pub use table::TableMetadata;

/// This is the single representation of any supported metadata file.
///
/// It uses the "name" field in the json to disambiguate between matrix tables, tables, and the
/// various types of component.
///
/// Note: some metadata elements have custom deserialisation implemented in the
/// [parse::schema](crate::parse::schema) module to allow the virtual and encoded schemas to be
/// parsed along with the document.
/// Note 2: A generalised [ComponentMetadata] type is also present to harmonise the various
/// component metadata formats. The other component metadata types implement
/// [Into]<ComponentMetadata>.
#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "name")]
pub enum Metadata {
    MatrixTableSpec(matrix::MatrixMetadata),

    TableSpec(table::TableMetadata),

    OrderedRVDSpec(component_1::RvdMetadataV1),
    IndexedRVDSpec(component_1::RvdMetadataV1),
    UnpartitionedRVDSpec(component_1::UnpartitionedRvdMetadataV1),

    OrderedRVDSpec2(component_2::RVDMetadataV2),
    IndexedRVDSpec2(component_2::RVDMetadataV2),
}
