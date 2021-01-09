// use jtd_codegen::target::*;

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
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::PascalCase)
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::PascalCase)
        ));
}

pub struct Target {
    namespace: String,
}

impl Target {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::FilePerType("cs".into()),
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
            strings_are_nullable: true,
            timestamps_are_nullable: false,
            arrays_are_nullable: true,
            dicts_are_nullable: true,
            aliases_are_nullable: true,
            enums_are_nullable: false,
            structs_are_nullable: true,
            discriminators_are_nullable: true,
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
        if let Some(s) = metadata
            .get("csharpSystemTextType")
            .and_then(|v| v.as_str())
        {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "object".into(),
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "sbyte".into(),
            target::Expr::Uint8 => "byte".into(),
            target::Expr::Int16 => "short".into(),
            target::Expr::Uint16 => "ushort".into(),
            target::Expr::Int32 => "int".into(),
            target::Expr::Uint32 => "uint".into(),
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "double".into(),
            target::Expr::String => "string".into(),
            target::Expr::Timestamp => {
                state.imports.insert("System".into());
                "DateTimeOffset".into()
            }
            target::Expr::ArrayOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("csharpSystemTextContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<{}>", s, sub_expr);
                }

                state.imports.insert("System.Collections.Generic".into());
                format!("IList<{}>", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("csharpSystemTextContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<string, {}>", s, sub_expr);
                }

                state.imports.insert("System.Collections.Generic".into());
                format!("IDictionary<string, {}>", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => format!("{}?", sub_expr),
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
                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(
                    out,
                    "{}",
                    AliasTemplate {
                        namespace: &self.namespace,
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
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(
                    out,
                    "{}",
                    EnumTemplate {
                        namespace: &self.namespace,
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
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .insert("System.Text.Json.Serialization".into());

                writeln!(
                    out,
                    "{}",
                    StructTemplate {
                        namespace: &self.namespace,
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
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(
                    out,
                    "{}",
                    DiscriminatorTemplate {
                        namespace: &self.namespace,
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
                tag_field_name,
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .insert("System.Text.Json.Serialization".into());

                writeln!(
                    out,
                    "{}",
                    DiscriminatorVariantTemplate {
                        namespace: &self.namespace,
                        metadata: &metadata,
                        name: &name,
                        parent_name: &parent_name,
                        tag_field_name: &tag_field_name,
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
    imports: BTreeSet<String>,
}

#[derive(Template)]
#[template(path = "preamble")]
struct PreambleTemplate<'a> {
    imports: &'a BTreeSet<String>,
}

#[derive(Template)]
#[template(path = "alias")]
struct AliasTemplate<'a> {
    namespace: &'a str,
    metadata: &'a metadata::Metadata,
    name: &'a str,
    type_: &'a str,
}

#[derive(Template)]
#[template(path = "enum")]
struct EnumTemplate<'a> {
    namespace: &'a str,
    metadata: &'a metadata::Metadata,
    name: &'a str,
    members: &'a [target::EnumMember],
}

#[derive(Template)]
#[template(path = "struct")]
struct StructTemplate<'a> {
    namespace: &'a str,
    metadata: &'a metadata::Metadata,
    name: &'a str,
    fields: &'a [target::Field],
}

#[derive(Template)]
#[template(path = "discriminator")]
struct DiscriminatorTemplate<'a> {
    namespace: &'a str,
    metadata: &'a metadata::Metadata,
    name: &'a str,
    tag_field_name: &'a str,
    tag_json_name: &'a str,
    variants: &'a [target::DiscriminatorVariantInfo],
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    namespace: &'a str,
    metadata: &'a metadata::Metadata,
    name: &'a str,
    parent_name: &'a str,
    tag_field_name: &'a str,
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
            &format!("{}/// <summary>", prefix),
            &format!("{}/// ", prefix),
            &format!("{}/// </summary>", prefix),
            s,
        )
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("JtdCodegenE2E".into()));
    }
}
