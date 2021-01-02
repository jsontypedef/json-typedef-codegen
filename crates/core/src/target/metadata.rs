use serde_json::Value;
use std::collections::BTreeMap;

pub type Metadata = BTreeMap<String, Value>;

pub fn description(metadata: &Metadata) -> &str {
    metadata
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default()
}

pub fn enum_variant_description<'a>(metadata: &'a Metadata, value: &'a str) -> &'a str {
    metadata
        .get("enumDescription")
        .and_then(Value::as_object)
        .and_then(|m| m.get(value))
        .and_then(Value::as_str)
        .unwrap_or_default()
}
