// use jtd_codegen::target::*;

use askama::Template;
use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
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
            inflect::TailInflector::new(inflect::Case::screaming_snake_case())
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
            file_partitioning: target::FilePartitioningStrategy::SingleFile("__init__.py".into()),
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
        if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Any".into());

                "Any".into()
            }
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "int".into(),
            target::Expr::Uint8 => "int".into(),
            target::Expr::Int16 => "int".into(),
            target::Expr::Uint16 => "int".into(),
            target::Expr::Int32 => "int".into(),
            target::Expr::Uint32 => "int".into(),
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "float".into(),
            target::Expr::String => "str".into(),
            target::Expr::Timestamp => "str".into(),
            target::Expr::ArrayOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("List".into());

                format!("List[{}]", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Dict".into());

                format!("Dict[str, {}]", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Optional".into());

                format!("Optional[{}]", sub_expr)
            }
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
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .extend(vec![
                        "Any".into(),
                        "Union".into(),
                        "get_origin".into(),
                        "get_args".into(),
                    ]);

                writeln!(
                    out,
                    "{}",
                    PreambleTemplate {
                        imports: &state.imports
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                writeln!(
                    out,
                    "{}",
                    AliasTemplate {
                        metadata: &metadata,
                        name: &name,
                        type_: &type_,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("enum".into())
                    .or_default()
                    .insert("Enum".into());

                writeln!(
                    out,
                    "{}",
                    EnumTemplate {
                        metadata: &metadata,
                        name: &name,
                        members: &members,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Optional".into());

                writeln!(
                    out,
                    "{}",
                    StructTemplate {
                        metadata: &metadata,
                        name: &name,
                        fields: &fields,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                tag_field_name,
                tag_json_name,
                variants,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                writeln!(
                    out,
                    "{}",
                    DiscriminatorTemplate {
                        metadata: &metadata,
                        name: &name,
                        tag_field_name: &tag_field_name,
                        tag_json_name: &tag_json_name,
                        variants: &variants,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                parent_name,
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                writeln!(
                    out,
                    "{}",
                    DiscriminatorVariantTemplate {
                        metadata: &metadata,
                        name: &name,
                        parent_name: &parent_name,
                        tag_json_name: &tag_json_name,
                        tag_value: &tag_value,
                        fields: &fields,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Template)]
#[template(path = "preamble")]
struct PreambleTemplate<'a> {
    imports: &'a BTreeMap<String, BTreeSet<String>>,
}

#[derive(Template)]
#[template(path = "alias")]
struct AliasTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    type_: &'a str,
}

#[derive(Template)]
#[template(path = "enum")]
struct EnumTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    members: &'a [target::EnumMember],
}

#[derive(Template)]
#[template(path = "struct")]
struct StructTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    fields: &'a [target::Field],
}

#[derive(Template)]
#[template(path = "discriminator")]
struct DiscriminatorTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    tag_field_name: &'a str,
    tag_json_name: &'a str,
    variants: &'a [target::DiscriminatorVariantInfo],
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    parent_name: &'a str,
    tag_json_name: &'a str,
    tag_value: &'a str,
    fields: &'a [target::Field],
}

mod filters {
    use askama::Result;
    use serde_json::Value;
    use std::collections::BTreeMap;

    pub fn description(metadata: &BTreeMap<String, Value>, indent: &usize) -> Result<String> {
        Ok(doc(
            *indent,
            jtd_codegen::target::metadata::description(metadata),
        ))
    }

    pub fn enum_variant_description(
        metadata: &BTreeMap<String, Value>,
        indent: &usize,
        value: &str,
    ) -> Result<String> {
        Ok(doc(
            *indent,
            jtd_codegen::target::metadata::enum_variant_description(metadata, value),
        ))
    }

    fn doc(ident: usize, s: &str) -> String {
        let prefix = "    ".repeat(ident);
        jtd_codegen::target::fmt::comment_block(
            &format!("{}\"\"\"", prefix),
            &format!("{}", prefix),
            &format!("{}\"\"\"", prefix),
            s,
        )
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new());
    }
}
