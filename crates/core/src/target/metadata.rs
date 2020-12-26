use serde_json::Value;
use std::collections::BTreeMap;

pub fn description(metadata: &BTreeMap<String, Value>) -> &str {
    metadata
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default()
}

pub fn enum_variant_description<'a>(metadata: &'a BTreeMap<String, Value>, value: &str) -> &'a str {
    metadata
        .get("enumDescription")
        .and_then(Value::as_object)
        .and_then(|m| m.get(value))
        .and_then(Value::as_str)
        .unwrap_or_default()
}
