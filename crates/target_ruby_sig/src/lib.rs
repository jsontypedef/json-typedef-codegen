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
    static ref INITIALISMS: BTreeSet<String> = include_str!("initialisms")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref FILE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::snake_case()),
        ));
    static ref MODULE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case_with_initialisms(
                INITIALISMS.clone()
            ))
        ));
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case_with_initialisms(
                INITIALISMS.clone()
            ))
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

pub struct Target {
    module: String,
}

impl Target {
    pub fn new(module: String) -> Self {
        Self {
            module: MODULE_NAMING_CONVENTION.inflect(&[module.into()]),
        }
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile(format!(
                "{}.rbs",
                FILE_NAMING_CONVENTION.inflect(&[self.module.clone()]),
            )),
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
        _state: &mut FileState,
        metadata: metadata::Metadata,
        expr: target::Expr,
    ) -> String {
        if let Some(s) = metadata.get("rubyType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "untyped".into(),
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "Integer".into(),
            target::Expr::Uint8 => "Integer".into(),
            target::Expr::Int16 => "Integer".into(),
            target::Expr::Uint16 => "Integer".into(),
            target::Expr::Int32 => "Integer".into(),
            target::Expr::Uint32 => "Integer".into(),
            target::Expr::Float32 => "Float".into(),
            target::Expr::Float64 => "Float".into(),
            target::Expr::String => "String".into(),
            target::Expr::Timestamp => "String".into(),
            target::Expr::ArrayOf(sub_expr) => {
                format!("Array[{}]", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                format!("Hash[String, {}]", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => format!("{}?", sub_expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        _state: &mut FileState,
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
                    "# Code generated by jtd-codegen for Ruby Type Signatures v{}",
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(out)?;
                writeln!(out, "module {}", self.module)?;

                None
            }

            target::Item::Postamble => {
                writeln!(out, "end")?;

                None
            }

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                writeln!(out)?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor value: {}", type_)?;
                writeln!(out)?;
                writeln!(out, "    def self.from_json: (data: untyped) -> {}", name)?;
                writeln!(out, "    def to_json: () -> untyped")?;
                writeln!(out, "  end")?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata.get("rubyType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor value: String")?;
                writeln!(out)?;
                for (index, member) in members.iter().enumerate() {
                    writeln!(out, "    {}: {}", member.name, name)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json: (data: untyped) -> {}", name)?;
                writeln!(out, "    def to_json: () -> untyped")?;
                writeln!(out, "  end")?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata.get("rubyType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                writeln!(out, "  class {}", name)?;
                for field in &fields {
                    writeln!(out, "    attr_accessor {}: {}", field.name, field.type_)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json: (data: untyped) -> {}", name)?;
                writeln!(out, "    def to_json: () -> untyped")?;
                writeln!(out, "  end")?;

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                tag_field_name,
                tag_json_name,
                variants,
            } => {
                if let Some(s) = metadata.get("rubyType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor {}: String", tag_field_name)?;
                writeln!(out)?;
                writeln!(out, "    def self.from_json: (data: untyped) -> {}", name)?;
                writeln!(out, "    def to_json: () -> untyped")?;
                writeln!(out, "  end")?;

                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                parent_name,
                tag_json_name,
                tag_field_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("rubyType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                writeln!(out, "  class {} < {}", name, parent_name)?;
                for field in &fields {
                    writeln!(out, "    attr_accessor {}: {}", field.name, field.type_)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json: (data: untyped) -> {}", name)?;
                writeln!(out, "    def to_json: () -> untyped")?;
                writeln!(out, "  end")?;

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("jtd_codegen_e2e".into()));
    }
}
