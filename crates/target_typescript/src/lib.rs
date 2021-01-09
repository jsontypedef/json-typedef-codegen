use askama::Template;
use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::PascalCase)
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::PascalCase)
        ));
}

pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = ();

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("index.ts".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::NativeSupport,
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
            target::NameableKind::EnumMember => ENUM_MEMBER_NAMING_CONVENTION.inflect(parts),

            // Not used. TypeScript maps directly to the JSON data, so we don't
            // have the option of distinguishing the JSON name from the
            // TypeScript name
            target::NameableKind::Field => "".into(),
        }
    }

    fn expr(&self, _state: &mut (), metadata: metadata::Metadata, expr: target::Expr) -> String {
        if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "any".into(),
            target::Expr::Boolean => "boolean".into(),
            target::Expr::Int8 => "number".into(),
            target::Expr::Uint8 => "number".into(),
            target::Expr::Int16 => "number".into(),
            target::Expr::Uint16 => "number".into(),
            target::Expr::Int32 => "number".into(),
            target::Expr::Uint32 => "number".into(),
            target::Expr::Float32 => "number".into(),
            target::Expr::Float64 => "number".into(),
            target::Expr::String => "string".into(),
            target::Expr::Timestamp => "string".into(),
            target::Expr::ArrayOf(sub_expr) => format!("{}[]", sub_expr),
            target::Expr::DictOf(sub_expr) => format!("{{ [key: string]: {} }}", sub_expr),
            target::Expr::NullableOf(sub_expr) => format!("({} | null)", sub_expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        _state: &mut (),
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Preamble => {
                // No need for a preamble for TypeScript, because we never have
                // any imports or similar things.
                None
            }

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
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
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

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
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

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
                variants,
                ..
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(
                    out,
                    "{}",
                    DiscriminatorTemplate {
                        metadata: &metadata,
                        name: &name,
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
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(
                    out,
                    "{}",
                    DiscriminatorVariantTemplate {
                        metadata: &metadata,
                        name: &name,
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

#[derive(Template)]
#[template(path = "preamble")]
struct PreambleTemplate {}

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
    variants: &'a [target::DiscriminatorVariantInfo],
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
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
            &format!("{}/**", prefix),
            &format!("{} * ", prefix),
            &format!("{} */", prefix),
            s,
        )
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("jtd_codegen_e2e".into()));
    }
}
