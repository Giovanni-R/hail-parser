use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::types::{metadata::table::TableSchema, VType, VTypeShape};

use crate::parse::schema::error::SchemaParsingError;

use super::helpers;

const _REGEX_PATTERN: &str = r"(?x)  # x flag enables comments and removes whitespace.
    Table\{
        (global:\+?Struct\{.*\}),
        key:\[(.*)\],
        (row:\+?Struct\{.*\})
    \}";
static TABLE_SCHEMA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(_REGEX_PATTERN).unwrap());

impl<'de> Deserialize<'de> for TableSchema {
    fn deserialize<D>(deserializer: D) -> Result<TableSchema, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = String::deserialize(deserializer)?;

        string_to_table_schema(intermediate).map_err(serde::de::Error::custom)
    }
}

fn string_to_table_schema(value: String) -> Result<TableSchema, SchemaParsingError> {
    let captures: regex::Captures = TABLE_SCHEMA_REGEX.captures(&value).ok_or_else(|| {
        SchemaParsingError::Generic(format!("Unable to capture table schema in: {}", value))
    })?;

    let global_schema =
        helpers::extract_field_from_regex_match(&captures.get(1), "global", &value)?;

    let key_names = helpers::extract_keys_from_regex_match(&captures.get(2), &value)?;

    let row_schema = helpers::extract_field_from_regex_match(&captures.get(3), "row", &value)?;

    match row_schema {
        VType {
            shape: VTypeShape::Struct(ref row_fields),
            ..
        } => {
            let row_keys = helpers::get_key_with_types(&key_names, row_fields);

            Ok(TableSchema {
                global_schema,
                row_schema,
                row_keys,
            })
        }
        _ => Err(SchemaParsingError::Generic(format!(
            "Parsed table schema is not a Struct: {}",
            value
        ))),
    }
}
