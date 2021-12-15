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
            inflect::CombiningInflector::new(inflect::Case::snake_case()),
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
        Self { module }
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile(format!(
                "{}.rb",
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
            target::Expr::Empty => "Object".into(),
            target::Expr::Boolean => "TrueClass".into(),
            target::Expr::Int8 => "Integer".into(),
            target::Expr::Uint8 => "Integer".into(),
            target::Expr::Int16 => "Integer".into(),
            target::Expr::Uint16 => "Integer".into(),
            target::Expr::Int32 => "Integer".into(),
            target::Expr::Uint32 => "Integer".into(),
            target::Expr::Float32 => "Float".into(),
            target::Expr::Float64 => "Float".into(),
            target::Expr::String => "String".into(),
            target::Expr::Timestamp => "DateTime".into(),
            target::Expr::ArrayOf(sub_expr) => {
                format!("Array[{}]", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                format!("Hash[String, {}]", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => sub_expr,
            target::Expr::RecursiveRef(sub_expr) => sub_expr,
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
                    "# Code generated by jtd-codegen for Ruby v{}",
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(out)?;
                writeln!(out, "require 'json'")?;
                writeln!(out, "require 'time'")?;
                writeln!(out)?;
                writeln!(out, "module {}", self.module)?;

                None
            }

            target::Item::Postamble => {
                writeln!(out)?;
                writeln!(out, "  private")?;
                writeln!(out)?;
                writeln!(out, "  def self.from_json_data(type, data)")?;
                writeln!(out, "    if data.nil? || [Object, TrueClass, Integer, Float, String].include?(type)")?;
                writeln!(out, "      data")?;
                writeln!(out, "    elsif type == DateTime")?;
                writeln!(out, "      DateTime.rfc3339(data)")?;
                writeln!(out, "    elsif type.is_a?(Array)")?;
                writeln!(
                    out,
                    "      data.map {{ |elem| from_json_data(type.first, elem) }}"
                )?;
                writeln!(out, "    elsif type.is_a?(Hash)")?;
                writeln!(
                    out,
                    "      data.transform_values {{ |elem| from_json_data(type.values.first, elem) }}"
                )?;
                writeln!(out, "    else")?;
                writeln!(out, "      type.from_json_data(data)")?;
                writeln!(out, "    end")?;
                writeln!(out, "  end")?;
                writeln!(out)?;
                writeln!(out, "  def self.to_json_data(data)")?;
                writeln!(out, "    if data.nil? || [TrueClass, FalseClass, Integer, Float, String].include?(data.class)")?;
                writeln!(out, "      data")?;
                writeln!(out, "    elsif data.is_a?(DateTime)")?;
                writeln!(out, "      data.rfc3339")?;
                writeln!(out, "    elsif data.is_a?(Array)")?;
                writeln!(out, "      data.map {{ |elem| to_json_data(elem) }}")?;
                writeln!(out, "    elsif data.is_a?(Hash)")?;
                writeln!(
                    out,
                    "      data.transform_values {{ |elem| to_json_data(elem) }}"
                )?;
                writeln!(out, "    else")?;
                writeln!(out, "      data.to_json_data")?;
                writeln!(out, "    end")?;
                writeln!(out, "  end")?;
                writeln!(out, "end")?;

                None
            }

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                writeln!(out)?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor :value")?;
                writeln!(out)?;
                writeln!(out, "    def self.from_json_data(data)")?;
                writeln!(out, "      out = {}.new", name)?;
                writeln!(
                    out,
                    "      out.value = {}.from_json_data({}, data)",
                    self.module, type_
                )?;
                writeln!(out, "      out")?;
                writeln!(out, "    end")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data")?;
                writeln!(out, "      {}.to_json_data(value)", self.module)?;
                writeln!(out, "    end")?;
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
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor :value")?;
                writeln!(out)?;
                writeln!(out, "    def initialize(value)")?;
                writeln!(out, "      self.value = value")?;
                writeln!(out, "    end")?;
                writeln!(out)?;
                writeln!(out, "    private_class_method :new")?;
                writeln!(out)?;
                for (index, member) in members.iter().enumerate() {
                    let description = enum_variant_description(&metadata, 2, &member.json_value);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }
                    write!(out, "{}", description)?;
                    writeln!(out, "    {} = new({:?})", member.name, member.json_value)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json_data(data)")?;
                writeln!(out, "      {{")?;
                for member in &members {
                    writeln!(out, "        {:?} => {},", member.json_value, member.name)?;
                }
                writeln!(out, "      }}[data]")?;
                writeln!(out, "    end")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data")?;
                writeln!(out, "      value")?;
                writeln!(out, "    end")?;
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
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "  class {}", name)?;
                for (index, field) in fields.iter().enumerate() {
                    let description = description(&field.metadata, 2);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }
                    write!(out, "{}", description)?;
                    writeln!(out, "    attr_accessor :{}", field.name)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json_data(data)")?;
                writeln!(out, "      out = {}.new", name)?;
                for field in &fields {
                    writeln!(
                        out,
                        "      out.{} = {}::from_json_data({}, data[{:?}])",
                        field.name, self.module, field.type_, field.json_name
                    )?;
                }
                writeln!(out, "      out")?;
                writeln!(out, "    end")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data")?;
                writeln!(out, "      data = {{}}")?;
                for field in &fields {
                    if field.optional {
                        writeln!(
                            out,
                            "      data[{:?}] = {}::to_json_data({}) unless {}.nil?",
                            field.json_name, self.module, field.name, field.name
                        )?;
                    } else {
                        writeln!(
                            out,
                            "      data[{:?}] = {}::to_json_data({})",
                            field.json_name, self.module, field.name
                        )?;
                    }
                }
                writeln!(out, "      data")?;
                writeln!(out, "    end")?;
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
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "  class {}", name)?;
                writeln!(out, "    attr_accessor :{}", tag_field_name)?;
                writeln!(out)?;
                writeln!(out, "    def self.from_json_data(data)")?;
                writeln!(out, "      {{")?;
                for variant in &variants {
                    writeln!(
                        out,
                        "        {:?} => {},",
                        variant.tag_value, variant.type_name
                    )?;
                }
                writeln!(
                    out,
                    "      }}[data[{:?}]].from_json_data(data)",
                    tag_json_name
                )?;
                writeln!(out, "    end")?;
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
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "  class {} < {}", name, parent_name)?;
                for field in &fields {
                    writeln!(out, "    attr_accessor :{}", field.name)?;
                }
                writeln!(out)?;
                writeln!(out, "    def self.from_json_data(data)")?;
                writeln!(out, "      out = {}.new", name)?;
                writeln!(out, "      out.{} = {:?}", tag_field_name, tag_value)?;
                for field in &fields {
                    writeln!(
                        out,
                        "      out.{} = {}::from_json_data({}, data[{:?}])",
                        field.name, self.module, field.type_, field.json_name
                    )?;
                }
                writeln!(out, "      out")?;
                writeln!(out, "    end")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data")?;
                writeln!(
                    out,
                    "      data = {{ {:?} => {:?} }}",
                    tag_json_name, tag_value
                )?;
                for field in &fields {
                    if field.optional {
                        writeln!(
                            out,
                            "      data[{:?}] = {}::to_json_data({}) unless {}.nil?",
                            field.json_name, self.module, field.name, field.name
                        )?;
                    } else {
                        writeln!(
                            out,
                            "      data[{:?}] = {}::to_json_data({})",
                            field.json_name, self.module, field.name
                        )?;
                    }
                }
                writeln!(out, "      data")?;
                writeln!(out, "    end")?;
                writeln!(out, "  end")?;

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {}

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
    let prefix = "  ".repeat(ident);
    jtd_codegen::target::fmt::comment_block("", &format!("{}# ", prefix), "", s)
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("JTDCodegenE2E".into()));
    }

    mod optional_std_tests {
        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new("JTDCodegenE2E".into()),
            empty_and_nonascii_properties
        );

        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new("JTDCodegenE2E".into()),
            empty_and_nonascii_enum_values
        );
    }
}
