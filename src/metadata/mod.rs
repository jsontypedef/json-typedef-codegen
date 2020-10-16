use jtd::Schema;

pub fn description(schema: &Schema) -> String {
    schema
        .metadata
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned()
}

// pub fn enum_description(schema: &Schema, name: &str) -> String {
//     schema
//         .metadata
//         .get("enumDescriptions")
//         .and_then(|v| v.as_object())
//         .and_then(|a| a.get(name))
//         .and_then(|v| v.as_str())
//         .unwrap_or_default()
//         .to_owned()
// }
