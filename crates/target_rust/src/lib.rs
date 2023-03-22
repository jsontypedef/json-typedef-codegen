use jtd_codegen::error::Error;
use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case())
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::snake_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
}

pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("mod.rs".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::WrapWithNullable,
            booleans_are_nullable: false,
            int8s_are_nullable: false,
            uint8s_are_nullable: false,
            int16s_are_nullable: false,
            uint16s_are_nullable: false,
            int32s_are_nullable: false,
            uint32s_are_nullable: false,
            float32s_are_nullable: false,
            float64s_are_nullable: false,
            strings_are_nullable: false,
            timestamps_are_nullable: false,
            arrays_are_nullable: false,
            dicts_are_nullable: false,
            aliases_are_nullable: false,
            enums_are_nullable: false,
            structs_are_nullable: false,
            discriminators_are_nullable: false,
        }
    }

    fn name(&self, kind: target::NameableKind, parts: &[String]) -> String {
        match kind {
            target::NameableKind::Type => TYPE_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::Field => FIELD_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::EnumMember => ENUM_MEMBER_NAMING_CONVENTION.inflect(parts),
        }
    }

    fn expr(
        &self,
        state: &mut FileState,
        metadata: metadata::Metadata,
        expr: target::Expr,
    ) -> String {
        if let Some(s) = metadata.get("rustType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => {
                state
                    .imports
                    .entry("serde_json".into())
                    .or_default()
                    .insert("Value".to_owned());

                "Option<Value>".into()
            }
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "i8".into(),
            target::Expr::Uint8 => "u8".into(),
            target::Expr::Int16 => "i16".into(),
            target::Expr::Uint16 => "u16".into(),
            target::Expr::Int32 => "i32".into(),
            target::Expr::Uint32 => "u32".into(),
            target::Expr::Float32 => "f32".into(),
            target::Expr::Float64 => "f64".into(),
            target::Expr::String => "String".into(),
            target::Expr::Timestamp => {
                state
                    .imports
                    .entry("chrono".into())
                    .or_default()
                    .extend(vec!["DateTime".to_owned(), "FixedOffset".to_owned()]);

                "DateTime<FixedOffset>".into()
            }
            target::Expr::ArrayOf(sub_expr) => format!("Vec<{}>", sub_expr),
            target::Expr::DictOf(sub_expr) => {
                state
                    .imports
                    .entry("std::collections".into())
                    .or_default()
                    .insert("HashMap".to_owned());

                format!("HashMap<String, {}>", sub_expr)
            }

            target::Expr::NullableOf(sub_expr) => format!("Option<{}>", sub_expr),
            // A Box here is usually necessary for recursive data structures,
            // such as in the geojson test case.
            //
            // Note that this strategy is slighyly over-defensive;
            // in a cycle of mutually recursive types,
            // only one of the types needs to be boxed to break the cycle.
            // In such cases, the user may want to optimize the code,
            // overriding some of the boxings with metadata.rustType.
            target::Expr::RecursiveRef(sub_expr) => format!("Box<{}>", sub_expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut FileState,
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Auxiliary { .. } => {
                // No auxiliary files needed.
                None
            }

            target::Item::Preamble => {
                writeln!(
                    out,
                    "// Code generated by jtd-codegen for Rust v{}",
                    env!("CARGO_PKG_VERSION")
                )?;

                if !state.imports.is_empty() {
                    writeln!(out)?;
                }

                for (module, idents) in &state.imports {
                    write!(out, "use {}::", module)?;

                    let idents: Vec<String> = idents.iter().cloned().collect();
                    if idents.len() == 1 {
                        write!(out, "{}", &idents[0])?;
                    } else {
                        write!(out, "{{{}}}", idents.join(", "))?;
                    }

                    writeln!(out, ";")?;
                }

                None
            }

            target::Item::Postamble => None,

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "pub type {} = {};", name, type_)?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata.get("rustType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("serde".into())
                    .or_default()
                    .extend(vec!["Deserialize".to_owned(), "Serialize".to_owned()]);

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "#[derive(Serialize, Deserialize)]")?;
                writeln!(out, "pub enum {} {{", name)?;

                for (index, member) in members.into_iter().enumerate() {
                    if index != 0 {
                        writeln!(out)?;
                    }

                    write!(
                        out,
                        "{}",
                        enum_variant_description(&metadata, 1, &member.json_value)
                    )?;
                    writeln!(out, "    #[serde(rename = {:?})]", member.json_value)?;
                    writeln!(out, "    {},", member.name)?;
                }

                writeln!(out, "}}")?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata.get("rustType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                let mut derives = vec!["Serialize", "Deserialize"];

                if let Some(s) = metadata.get("rustCustomDerive").and_then(|v| v.as_str()) {
                    derives.extend(s.split(","));
                }

                state
                    .imports
                    .entry("serde".into())
                    .or_default()
                    .extend(vec!["Deserialize".to_owned(), "Serialize".to_owned()]);

                let mut custom_use = Vec::<&str>::new();
                if let Some(s) = metadata.get("rustCustomUse").and_then(|v| v.as_str()) {
                    custom_use.extend(s.split(";"));
                }
                for cu in custom_use {
                    // custom::path::{import,export} or custom::path::single
                    let mut use_imports = Vec::<&str>::new();
                    let mut path_parts = cu.split("::").collect::<Vec<&str>>();
                    let mut last_part = path_parts.pop().unwrap();
                    // If there are no path_parts or the last part was "", panic!
                    if path_parts.len() < 1 || last_part.trim().len() < 1 {
                        return Err(Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Invalid custom use statement: {:?}", cu),
                        )));
                    }
                    if last_part.starts_with('{') {
                        // Strip the first/last chars and split
                        last_part = &last_part[1..last_part.len() - 1];
                        use_imports.extend(last_part.split(","))
                    } else {
                        // No, just push it into the imports list
                        use_imports.push(last_part);
                    }
                    state
                        .imports
                        .entry(path_parts.join("::").into())
                        .or_default()
                        .extend(use_imports.drain(..).map(|i| i.trim().to_owned()));
                }

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "#[derive({})]", derives.join(", "))?;

                if fields.is_empty() {
                    writeln!(out, "pub struct {} {{}}", name)?;
                } else {
                    writeln!(out, "pub struct {} {{", name)?;
                    for (index, field) in fields.into_iter().enumerate() {
                        if index != 0 {
                            writeln!(out)?;
                        }

                        write!(out, "{}", description(&field.metadata, 1))?;
                        writeln!(out, "    #[serde(rename = {:?})]", field.json_name)?;
                        if field.optional {
                            writeln!(
                                out,
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                            )?;
                        }
                        writeln!(out, "    pub {}: {},", field.name, field.type_)?;
                    }

                    writeln!(out, "}}")?;
                }

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                variants,
                tag_json_name,
                ..
            } => {
                if let Some(s) = metadata.get("rustType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("serde".into())
                    .or_default()
                    .extend(vec!["Deserialize".to_owned(), "Serialize".to_owned()]);

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "#[derive(Serialize, Deserialize)]")?;
                writeln!(out, "#[serde(tag = {:?})]", tag_json_name)?;
                writeln!(out, "pub enum {} {{", name)?;

                for (index, variant) in variants.into_iter().enumerate() {
                    if index != 0 {
                        writeln!(out)?;
                    }

                    writeln!(out, "    #[serde(rename = {:?})]", variant.tag_value)?;
                    writeln!(
                        out,
                        "    {}({}),",
                        inflect::Case::pascal_case().inflect(&[variant.field_name]),
                        variant.type_name
                    )?;
                }

                writeln!(out, "}}")?;

                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("rustType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("serde".into())
                    .or_default()
                    .extend(vec!["Deserialize".to_owned(), "Serialize".to_owned()]);

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "#[derive(Serialize, Deserialize)]")?;

                if fields.is_empty() {
                    writeln!(out, "pub struct {} {{}}", name)?;
                } else {
                    writeln!(out, "pub struct {} {{", name)?;
                    for (index, field) in fields.into_iter().enumerate() {
                        if index != 0 {
                            writeln!(out)?;
                        }

                        write!(out, "{}", description(&field.metadata, 1))?;
                        writeln!(out, "    #[serde(rename = {:?})]", field.json_name)?;
                        if field.optional {
                            writeln!(
                                out,
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                            )?;
                        }
                        writeln!(out, "    pub {}: {},", field.name, field.type_)?;
                    }

                    writeln!(out, "}}")?;
                }

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeMap<String, BTreeSet<String>>,
}

fn description(metadata: &BTreeMap<String, Value>, indent: usize) -> String {
    doc(indent, jtd_codegen::target::metadata::description(metadata))
}

fn enum_variant_description(
    metadata: &BTreeMap<String, Value>,
    indent: usize,
    value: &str,
) -> String {
    doc(
        indent,
        jtd_codegen::target::metadata::enum_variant_description(metadata, value),
    )
}

fn doc(ident: usize, s: &str) -> String {
    let prefix = "    ".repeat(ident);
    jtd_codegen::target::fmt::comment_block("", &format!("{}/// ", prefix), "", s)
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new());
    }

    mod optional_std_tests {
        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_properties
        );

        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_enum_values
        );
    }
}
