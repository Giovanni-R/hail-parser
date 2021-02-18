use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::types::metadata::component_1::RvdTypeSchema;

use crate::parse::schema::error::SchemaParsingError;

use super::helpers;

const _REGEX_PATTERN: &str = r"(?x)  # x flag enables comments and removes whitespace.
    (?:Ordered)?RVDType\{
        key:\[(.*)\],
        (row:Struct\{.*\})
    \}";

static COMPONENT_SCHEMA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(_REGEX_PATTERN).unwrap());

impl<'de> Deserialize<'de> for RvdTypeSchema {
    fn deserialize<D>(deserializer: D) -> Result<RvdTypeSchema, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = String::deserialize(deserializer)?;

        string_to_rvd_type_schema(intermediate).map_err(serde::de::Error::custom)
    }
}

fn string_to_rvd_type_schema(value: String) -> Result<RvdTypeSchema, SchemaParsingError> {
    let captures: regex::Captures = COMPONENT_SCHEMA_REGEX.captures(&value).ok_or_else(|| {
        SchemaParsingError::Generic(format!("Unable to capture component schema in: {}", value))
    })?;

    let keys: Vec<String> = helpers::extract_keys_from_regex_match(&captures.get(1), &value)?;

    let row_schema = helpers::extract_field_from_regex_match(&captures.get(2), "row", &value)?;

    Ok(RvdTypeSchema {
        row_schema,
        row_keys: keys,
    })
}
