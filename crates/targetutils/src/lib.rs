use serde_json::Value;
use std::collections::BTreeMap;

pub fn get_metadata_str<'a>(key: &str, metadata: &'a BTreeMap<String, Value>) -> &'a str {
    metadata
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
}

pub fn description(metadata: &BTreeMap<String, Value>) -> &str {
    get_metadata_str("description", metadata)
}

pub fn enum_variant_description<'a>(metadata: &'a BTreeMap<String, Value>, value: &str) -> &'a str {
    metadata
        .get("enumDescription")
        .and_then(Value::as_object)
        .and_then(|m| m.get(value))
        .and_then(Value::as_str)
        .unwrap_or_default()
}

pub fn comment_block(before: &str, prefix: &str, after: &str, s: &str) -> String {
    let middle = textwrap::fill(s, 80 - prefix.len())
        .lines()
        .map(|s| format!("{}{}", prefix, s))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}\n{}\n", before, middle, after)
}
