use crate::types::VType;

use crate::parse::schema::{error::SchemaParsingError, SchemaFromString};

pub fn extract_keys_from_regex_match(
    raw_keys: &Option<regex::Match>,
    full_value: &str,
) -> Result<Vec<String>, SchemaParsingError> {
    Ok(raw_keys
        .ok_or(SchemaParsingError::Generic(format!(
            "Unable to extract keys from the schema: {}",
            full_value
        )))?
        .as_str()
        .trim_matches(&['[', ']'] as &[char])
        .split(',')
        .map(|s| s.to_owned())
        .collect())
}

pub fn extract_field_from_regex_match(
    raw_field: &Option<regex::Match>,
    field_name: &str,
    full_value: &str,
) -> Result<VType, SchemaParsingError> {
    let raw_fields = raw_field
        .ok_or(SchemaParsingError::Generic(format!(
            "Unable to extract the fields from the schema: {}",
            full_value
        )))?
        .as_str();

    let (found_field_name, field_schema) = VType::parse_named_type(raw_fields)?;

    assert!(field_name == found_field_name);

    Ok(field_schema)
}

pub fn get_key_with_types(
    key_names: &[String],
    fields: &[(String, VType)],
) -> Vec<(String, VType)> {
    fields
        .iter()
        .filter(|(key_name, _)| key_names.contains(key_name))
        .cloned()
        .collect()
}
