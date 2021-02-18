use serde::{Deserialize, Deserializer};

use crate::types::{EType, VType};

use crate::parse::schema::SchemaFromString;

impl<'de> Deserialize<'de> for EType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = String::deserialize(deserializer)?;

        Self::parse_type(&intermediate).map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for VType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = String::deserialize(deserializer)?;

        Self::parse_type(&intermediate).map_err(serde::de::Error::custom)
    }
}
