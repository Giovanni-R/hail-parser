use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::types::{metadata::matrix::MatrixSchema, VType, VTypeShape};

use crate::parse::schema::error::SchemaParsingError;

use super::helpers;

const _REGEX_PATTERN: &str = r"(?x)  # x flag enables comments and removes whitespace.
    Matrix\{
        (global:\+?Struct\{.*\}),
        col_key:\[(.*)\],
        (col:\+?Struct\{.*\}),
        row_key:\[(.*)\],
        (row:\+?Struct\{.*\}),
        (entry:\+?Struct\{.*\})
    \}";
static MATRIX_SCHEMA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(_REGEX_PATTERN).unwrap());

impl<'de> Deserialize<'de> for MatrixSchema {
    fn deserialize<D>(deserializer: D) -> Result<MatrixSchema, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = String::deserialize(deserializer)?;

        string_to_matrix_schema(intermediate).map_err(serde::de::Error::custom)
    }
}

fn string_to_matrix_schema(value: String) -> Result<MatrixSchema, SchemaParsingError> {
    let captures: regex::Captures = MATRIX_SCHEMA_REGEX.captures(&value).ok_or_else(|| {
        SchemaParsingError::Generic(format!("Unable to capture matrix schema in: {}", value))
    })?;

    let global_schema =
        helpers::extract_field_from_regex_match(&captures.get(1), "global", &value)?;

    let col_key_names = helpers::extract_keys_from_regex_match(&captures.get(2), &value)?;
    let col_schema = helpers::extract_field_from_regex_match(&captures.get(3), "col", &value)?;

    let row_key_names = helpers::extract_keys_from_regex_match(&captures.get(4), &value)?;
    let row_schema = helpers::extract_field_from_regex_match(&captures.get(5), "row", &value)?;

    let entry_schema = helpers::extract_field_from_regex_match(&captures.get(6), "entry", &value)?;

    match (&col_schema, &row_schema) {
        (
            VType {
                shape: VTypeShape::Struct(ref col_fields),
                ..
            },
            VType {
                shape: VTypeShape::Struct(ref row_fields),
                ..
            },
        ) => {
            let col_keys = helpers::get_key_with_types(&col_key_names, col_fields);
            let row_keys = helpers::get_key_with_types(&row_key_names, row_fields);

            Ok(MatrixSchema {
                global_schema,
                col_keys,
                col_schema,
                row_keys,
                row_schema,
                entry_schema,
            })
        }
        (_, _) => Err(SchemaParsingError::Generic(format!(
            "Parsed matrix column or row schema is not a Struct: {}",
            value
        ))),
    }
}
