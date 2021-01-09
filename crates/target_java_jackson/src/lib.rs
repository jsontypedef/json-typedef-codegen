// use jtd_codegen::target::*;

use askama::Template;
use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
            inflect::TailInflector::new(inflect::Case::CamelCase)
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::ScreamingSnakeCase)
        ));
}

pub struct Target {
    package: String,
}

impl Target {
    pub fn new(package: String) -> Self {
        Self { package }
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::FilePerType("java".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::WrapWithNullable,
            booleans_are_nullable: true,
            int8s_are_nullable: true,
            uint8s_are_nullable: true,
            int16s_are_nullable: true,
            uint16s_are_nullable: true,
            int32s_are_nullable: true,
            uint32s_are_nullable: true,
            float32s_are_nullable: true,
            float64s_are_nullable: true,
            strings_are_nullable: true,
            timestamps_are_nullable: true,
            arrays_are_nullable: true,
            dicts_are_nullable: true,
            aliases_are_nullable: true,
            enums_are_nullable: true,
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
        if let Some(s) = metadata.get("javaJacksonType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "Object".into(),
            target::Expr::Boolean => "Boolean".into(),
            target::Expr::Int8 => "Byte".into(),
            target::Expr::Uint8 => "UnsignedByte".into(),
            target::Expr::Int16 => "Short".into(),
            target::Expr::Uint16 => "UnsignedShort".into(),
            target::Expr::Int32 => "Integer".into(),
            target::Expr::Uint32 => "UnsignedInteger".into(),
            target::Expr::Float32 => "Float".into(),
            target::Expr::Float64 => "Double".into(),
            target::Expr::String => "String".into(),
            target::Expr::Timestamp => {
                state.imports.insert("java.time.OffsetDateTime".into());
                "OffsetDateTime".into()
            }
            target::Expr::ArrayOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("javaJacksonContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<{}>", s, sub_expr);
                }

                state.imports.insert("java.util.List".into());
                format!("List<{}>", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("javaJacksonContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<String, {}>", s, sub_expr);
                }

                state.imports.insert("java.util.Map".into());
                format!("Map<String, {}>", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => sub_expr, // everything is already nullable
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut FileState,
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Auxiliary { out_dir } => {
                let mut out = File::create(Path::join(&out_dir, "UnsignedByte.java"))?;
                writeln!(
                    out,
                    "{}",
                    UnsignedByteTemplate {
                        package: &self.package,
                    }
                    .render()
                    .unwrap()
                )?;

                let mut out = File::create(Path::join(&out_dir, "UnsignedShort.java"))?;
                writeln!(
                    out,
                    "{}",
                    UnsignedShortTemplate {
                        package: &self.package,
                    }
                    .render()
                    .unwrap()
                )?;

                let mut out = File::create(Path::join(&out_dir, "UnsignedInteger.java"))?;
                writeln!(
                    out,
                    "{}",
                    UnsignedIntegerTemplate {
                        package: &self.package,
                    }
                    .render()
                    .unwrap()
                )?;

                None
            }

            target::Item::Preamble => {
                writeln!(
                    out,
                    "{}",
                    PreambleTemplate {
                        package: &self.package,
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
                    "com.fasterxml.jackson.annotation.JsonValue".to_string(),
                    "com.fasterxml.jackson.annotation.JsonCreator".to_string(),
                ]);

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
                if let Some(s) = metadata.get("javaJacksonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .insert("com.fasterxml.jackson.annotation.JsonProperty".into());

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
                has_additional,
                fields,
            } => {
                if let Some(s) = metadata.get("javaJacksonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "com.fasterxml.jackson.annotation.JsonProperty".to_string(),
                    "com.fasterxml.jackson.databind.annotation.JsonSerialize".to_string(),
                ]);

                if has_additional {
                    state
                        .imports
                        .insert("com.fasterxml.jackson.annotation.JsonIgnoreProperties".into());
                }

                for field in &fields {
                    if field.optional {
                        state
                            .imports
                            .insert("com.fasterxml.jackson.annotation.JsonInclude".into());
                    }
                }

                writeln!(
                    out,
                    "{}",
                    StructTemplate {
                        metadata: &metadata,
                        name: &name,
                        has_additional,
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
                tag_json_name,
                variants,
                ..
            } => {
                if let Some(s) = metadata.get("javaJacksonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "com.fasterxml.jackson.annotation.JsonTypeInfo".to_string(),
                    "com.fasterxml.jackson.annotation.JsonSubTypes".to_string(),
                ]);

                writeln!(
                    out,
                    "{}",
                    DiscriminatorTemplate {
                        metadata: &metadata,
                        name: &name,
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
                has_additional,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("javaJacksonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "com.fasterxml.jackson.annotation.JsonProperty".to_string(),
                    "com.fasterxml.jackson.databind.annotation.JsonSerialize".to_string(),
                ]);

                if has_additional {
                    state
                        .imports
                        .insert("com.fasterxml.jackson.annotation.JsonIgnoreProperties".into());
                }

                for field in &fields {
                    if field.optional {
                        state
                            .imports
                            .insert("com.fasterxml.jackson.annotation.JsonInclude".into());
                    }
                }

                writeln!(
                    out,
                    "{}",
                    DiscriminatorVariantTemplate {
                        metadata: &metadata,
                        name: &name,
                        parent_name: &parent_name,
                        has_additional,
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
#[template(path = "UnsignedByte")]
struct UnsignedByteTemplate<'a> {
    package: &'a str,
}

#[derive(Template)]
#[template(path = "UnsignedShort")]
struct UnsignedShortTemplate<'a> {
    package: &'a str,
}

#[derive(Template)]
#[template(path = "UnsignedInteger")]
struct UnsignedIntegerTemplate<'a> {
    package: &'a str,
}

#[derive(Template)]
#[template(path = "preamble")]
struct PreambleTemplate<'a> {
    package: &'a str,
    imports: &'a BTreeSet<String>,
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
    has_additional: bool,
    fields: &'a [target::Field],
}

#[derive(Template)]
#[template(path = "discriminator")]
struct DiscriminatorTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    tag_json_name: &'a str,
    variants: &'a [target::DiscriminatorVariantInfo],
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    metadata: &'a metadata::Metadata,
    name: &'a str,
    parent_name: &'a str,
    has_additional: bool,
    fields: &'a [target::Field],
}

mod filters {
    use askama::Result;
    use serde_json::Value;
    use std::collections::BTreeMap;

    pub fn field_to_method_name(field_name: &str) -> Result<String> {
        let (head, tail) = field_name.split_at(1);
        Ok(head.to_uppercase() + tail)
    }

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
        jtd_codegen_test::std_test_cases!(&crate::Target::new("com.example".into()));
    }
}
