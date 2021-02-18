use crate::types::{encoding::VirtualHint, EType, ETypeShape, VType, VTypeShape};

/// Older components provide only the virtual schema, this function defines the default
/// encoded shape of each of the virtual primitives allowing the [EType] to be always used
/// for parsing.
pub fn virtual_type_to_default_encoded_type(virtual_type: &VType) -> EType {
    let required = virtual_type.required;

    let shape = match &virtual_type.shape {
        VTypeShape::Struct(inner_mapping) => ETypeShape::BaseStruct(
            inner_mapping
                .iter()
                .map(|(name, v_type)| {
                    (
                        name.to_owned(),
                        virtual_type_to_default_encoded_type(v_type),
                    )
                })
                .collect(),
        ),
        VTypeShape::Tuple(sequence_types) => ETypeShape::BaseStruct(
            sequence_types
                .iter()
                .zip(0u32..)
                .map(|(v_type, index)| {
                    (
                        format!("`{}`", index),
                        virtual_type_to_default_encoded_type(v_type),
                    )
                })
                .collect(),
        ),

        VTypeShape::Array(inner_type) => ETypeShape::Array(Box::new(
            virtual_type_to_default_encoded_type(inner_type.as_ref()),
        )),
        VTypeShape::Set(inner_type) => ETypeShape::Array(Box::new(
            virtual_type_to_default_encoded_type(inner_type.as_ref()),
        )),
        VTypeShape::Dict(virtual_key_type, virtual_value_type) => {
            let encoded_key_type = virtual_type_to_default_encoded_type(virtual_key_type);
            let encoded_value_type = virtual_type_to_default_encoded_type(virtual_value_type);

            ETypeShape::Array(Box::new(EType {
                shape: ETypeShape::BaseStruct(vec![
                    ("key".to_owned(), encoded_key_type),
                    ("value".to_owned(), encoded_value_type),
                ]),
                // Assume the element is always required
                // (explicit missing values in a dict make no sense)
                required: true,
                // No hint is necessary because the array will flag this as being a dict,
                // making the inner structure pre-determined.
                virtual_hint: None,
            }))
        }
        VTypeShape::NDArray(inner_type, n) => ETypeShape::NdArrayColumnMajor(
            Box::new(virtual_type_to_default_encoded_type(inner_type)),
            *n,
        ),

        VTypeShape::String => ETypeShape::Binary,

        VTypeShape::Float32 => ETypeShape::Float32,
        VTypeShape::Float64 => ETypeShape::Float64,
        VTypeShape::Int32 => ETypeShape::Int32,
        VTypeShape::Int64 => ETypeShape::Int64,

        VTypeShape::Boolean => ETypeShape::Boolean,

        VTypeShape::Call => ETypeShape::Int32,
        VTypeShape::Locus(_) => {
            let e_string = EType {
                shape: ETypeShape::Binary,
                required: true,
                virtual_hint: Some(VirtualHint::String),
            };
            let e_int32 = EType {
                shape: ETypeShape::Int32,
                required: true,
                virtual_hint: None,
            };
            ETypeShape::BaseStruct(vec![
                ("contig".to_owned(), e_string),
                ("position".to_owned(), e_int32),
            ])
        }
        VTypeShape::Interval(v_bounds_type) => {
            let mut e_bounds_type = virtual_type_to_default_encoded_type(v_bounds_type.as_ref());
            e_bounds_type.required = true;
            let e_boolean = EType {
                shape: ETypeShape::Boolean,
                required: true,
                virtual_hint: None,
            };

            ETypeShape::BaseStruct(vec![
                ("start".to_owned(), e_bounds_type.clone()),
                ("end".to_owned(), e_bounds_type),
                ("includesStart".to_owned(), e_boolean.clone()),
                ("includesEnd".to_owned(), e_boolean),
            ])
        }
    };

    EType {
        shape,
        required,
        virtual_hint: type_to_hint(Some(virtual_type)),
    }
}

/// While the [EType] is sufficient to successfully parse the data, virtual hints help the parser
/// correctly interpret the result into a useful [HailValue](crate::HailValue).
pub fn fill_encoded_type_with_virtual_hints(encoded: &mut EType, maybe_virtual: Option<&VType>) {
    // First add the hint
    encoded.virtual_hint = type_to_hint(maybe_virtual);

    // Then recursively insert the hint in any sub-types.
    let maybe_virtual_shape = maybe_virtual.map(|v| v.shape.clone());
    match (&mut encoded.shape, &maybe_virtual_shape) {
        // [Struct]
        // If the virtual type is also a struct, we need to pass the inner virtual types along
        // so that the encoded type can also be filled with the correct hints.
        (ETypeShape::BaseStruct(e_inner_mapping), Some(VTypeShape::Struct(v_inner_mapping))) => {
            let iter = e_inner_mapping
                .iter_mut()
                .zip(v_inner_mapping.iter().map(|(_, v)| v));
            for ((_, e), v) in iter {
                fill_encoded_type_with_virtual_hints(e, Some(v))
            }
        }
        // Interval as Struct
        // An interval is a struct with start and end, we need to make sure to fill those two
        // with the correct hint so that, for example, a Call interval would be parsed using two
        // HailValue::Call.
        (ETypeShape::BaseStruct(e_inner_mapping), Some(VTypeShape::Interval(bounds_type))) => {
            for (s, e) in e_inner_mapping.iter_mut() {
                match s.as_str() {
                    "start" | "end" => {
                        fill_encoded_type_with_virtual_hints(e, Some(bounds_type.as_ref()))
                    }
                    _ => fill_encoded_type_with_virtual_hints(e, None),
                }
            }
        }
        // Tuple as Struct
        (ETypeShape::BaseStruct(e_inner_mapping), Some(VTypeShape::Tuple(sequence_types))) => {
            for ((_, e), v) in e_inner_mapping.iter_mut().zip(sequence_types) {
                fill_encoded_type_with_virtual_hints(e, Some(v))
            }
        }
        // Locus as Struct
        // No special case necessary as the top-level hint is sufficient to direct to the
        // Locus-specific parsing function.
        // If we wanted to include it, then we would just need to make sure the contig string has
        // the correct hint (String).

        // [Array]
        // If the virtual type is also an array, then we need to pass the inner virtual type to
        // the inner encoded type so that it can also be filled with the correct hints.
        (ETypeShape::Array(inner_encoded), Some(VTypeShape::Array(inner_virtual))) => {
            fill_encoded_type_with_virtual_hints(
                inner_encoded.as_mut(),
                Some(inner_virtual.as_ref()),
            )
        }
        // Set as Array
        // Same as Array, but with different virtual hint.
        (ETypeShape::Array(inner_encoded), Some(VTypeShape::Set(inner_virtual))) => {
            fill_encoded_type_with_virtual_hints(
                inner_encoded.as_mut(),
                Some(inner_virtual.as_ref()),
            )
        }
        // Dict as Array
        // Same as Array, but with different virtual hint.
        // Note that a Dict is an Array[Struct([("key", HailType), ("value", HailType)])]
        (
            ETypeShape::Array(inner_encoded),
            Some(VTypeShape::Dict(virtual_key_type, virtual_value_type)),
        ) => fill_encoded_type_with_virtual_hints(
            inner_encoded.as_mut(),
            Some(&VType {
                shape: VTypeShape::Struct(vec![
                    ("key".to_owned(), *virtual_key_type.clone()),
                    ("value".to_owned(), *virtual_value_type.clone()),
                ]),
                required: true,
            }),
        ),

        // NDArray as NDArrayColumnMajor
        (
            ETypeShape::NdArrayColumnMajor(inner_encoded, _),
            Some(VTypeShape::NDArray(inner_virtual, _)),
        ) => fill_encoded_type_with_virtual_hints(
            inner_encoded.as_mut(),
            Some(inner_virtual.as_ref()),
        ),

        // Otherwise we can just update the virtual_hint.
        // No need to continue recursing because that only happens with structs and arrays.
        (_, _) => {}
    };
}

/// Maps the [VTypeShape]s with ambiguous behaviour to a [VirtualHint], the rest to None.
fn type_to_hint(maybe_virtual: Option<&VType>) -> Option<VirtualHint> {
    let maybe_virtual_shape = maybe_virtual.map(|v| &v.shape);
    match maybe_virtual_shape {
        Some(VTypeShape::Set(_)) => Some(VirtualHint::Set),
        Some(VTypeShape::Dict(_, _)) => Some(VirtualHint::Dict),
        Some(VTypeShape::String) => Some(VirtualHint::String),
        Some(VTypeShape::Call) => Some(VirtualHint::Call),
        Some(VTypeShape::Locus(genome)) => Some(VirtualHint::Locus(genome.clone())),
        Some(VTypeShape::Interval(_)) => Some(VirtualHint::Interval),
        Some(VTypeShape::Tuple(_)) => Some(VirtualHint::Tuple),
        _ => None,
    }
}
