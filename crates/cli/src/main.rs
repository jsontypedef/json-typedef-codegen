use jtd::{Schema, SerdeSchema};
use serde_json::json;
use std::convert::TryInto;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let schema: SerdeSchema = serde_json::from_value(json!({
        "definitions": {
            "thing": {
                "properties": {
                    "asdf": { "type": "boolean" },
                },
            },
        },
        "properties": {
            "foo": { "type": "boolean" },
            "things": { "elements": { "properties": { "id": {"type": "boolean"}}}},
            "quux": { "ref": "thing" },
        },
    }))?;

    let schema: Schema = schema
        .try_into()
        .expect("todo: make jtd's serdeconverterror a std error");

    let target = jtd_codegen_csharp_system_text::Target::new("JtdCodegen.Demo".into());
    jtd_codegen::codegen(&target, "Demo".into(), &schema, Path::new("foo"))?;

    Ok(())
}
