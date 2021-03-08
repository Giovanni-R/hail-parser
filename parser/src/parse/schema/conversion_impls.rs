use crate::types::metadata::{component, component_1, component_2, ComponentMetadata};

impl From<component_1::RvdMetadataV1> for ComponentMetadata {
    fn from(original: component_1::RvdMetadataV1) -> Self {
        let component_1::RvdMetadataV1 {
            rvd_type,
            codec_spec,
            part_files,
            ..
        } = original;

        let component_1::RvdTypeSchema {
            row_schema,
            row_keys,
        } = rvd_type;

        let encoded_type = super::helpers::virtual_type_to_default_encoded_type(&row_schema);

        let (std_codec_spec, buffer_spec) = match codec_spec {
            component_1::ComponentCodecSpec::PackCodecSpec { child: buffer_spec } => {
                (component::CodecSpec::PackCodecSpec, buffer_spec)
            }
        };

        ComponentMetadata {
            key: row_keys,
            virtual_type: row_schema,
            encoded_type,
            codec_spec: std_codec_spec,
            buffer_spec,
            part_files,
        }
    }
}

impl From<component_1::UnpartitionedRvdMetadataV1> for ComponentMetadata {
    fn from(original: component_1::UnpartitionedRvdMetadataV1) -> Self {
        let component_1::UnpartitionedRvdMetadataV1 {
            row_type,
            codec_spec,
            part_files,
            ..
        } = original;

        let encoded_type = super::helpers::virtual_type_to_default_encoded_type(&row_type);

        let (std_codec_spec, buffer_spec) = match codec_spec {
            component_1::ComponentCodecSpec::PackCodecSpec { child: buffer_spec } => {
                (component::CodecSpec::PackCodecSpec, buffer_spec)
            }
        };

        ComponentMetadata {
            key: vec![],
            virtual_type: row_type,
            encoded_type,
            codec_spec: std_codec_spec,
            buffer_spec,
            part_files,
        }
    }
}

impl From<component_2::RVDMetadataV2> for ComponentMetadata {
    fn from(original: component_2::RVDMetadataV2) -> Self {
        let component_2::RVDMetadataV2 {
            key,
            codec_spec,
            part_files,
        } = original;

        let (std_codec_spec, inner_buffer_spec) = match codec_spec {
            component_2::ComponentCodecSpecV2::TypedCodecSpec(inner) => {
                (component::CodecSpec::TypedCodecSpec, inner)
            }
        };

        let component_2::TypedCodecSpec {
            mut encoded_type,
            virtual_type,
            buffer_spec,
        } = inner_buffer_spec;

        super::helpers::fill_encoded_type_with_virtual_hints(
            &mut encoded_type,
            Some(&virtual_type),
        );

        ComponentMetadata {
            key,
            virtual_type,
            encoded_type,
            codec_spec: std_codec_spec,
            buffer_spec,
            part_files,
        }
    }
}
