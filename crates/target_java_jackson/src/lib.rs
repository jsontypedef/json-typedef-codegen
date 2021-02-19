use askama::Template;
use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
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
            inflect::CombiningInflector::new(inflect::Case::pascal_case())
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::camel_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::screaming_snake_case())
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
                        version: env!("CARGO_PKG_VERSION"),
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
                        version: env!("CARGO_PKG_VERSION"),
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
                        version: env!("CARGO_PKG_VERSION"),
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
                    "// Code generated by jtd-codegen for Java + Jackson v{}",
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(out)?;

                writeln!(out, "package {};", &self.package)?;

                if !state.imports.is_empty() {
                    writeln!(out)?;
                }

                for import in &state.imports {
                    writeln!(out, "import {};", import)?;
                }

                writeln!(out)?;

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

                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "public class {} {{", name)?;
                writeln!(out, "    @JsonValue")?;
                writeln!(out, "    private {} value;", type_)?;
                writeln!(out)?;
                writeln!(out, "    public {}() {{", name)?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
                writeln!(out, "    @JsonCreator")?;
                writeln!(out, "    public {}({} value) {{", name, type_)?;
                writeln!(out, "        this.value = value;")?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
                writeln!(out, "    public {} getValue() {{", type_)?;
                writeln!(out, "        return value;")?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
                writeln!(out, "    public void setValue({} value) {{", type_)?;
                writeln!(out, "        this.value = value;")?;
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

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

                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "public enum {} {{", name)?;
                for (index, member) in members.iter().enumerate() {
                    if index != 0 {
                        writeln!(out)?;
                    }

                    write!(
                        out,
                        "{}",
                        enum_variant_description(&metadata, 1, &member.json_value)
                    )?;
                    writeln!(out, "    @JsonProperty({:?})", member.json_value)?;
                    writeln!(out, "    {},", member.name)?;
                }
                writeln!(out, "}}")?;

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

                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "@JsonSerialize")?;
                if has_additional {
                    writeln!(out, "@JsonIgnoreProperties(ignoreUnknown = true)")?;
                }
                writeln!(out, "public class {} {{", name)?;
                for field in &fields {
                    if field.optional {
                        writeln!(out, "    @JsonInclude(JsonInclude.Include.NON_NULL)")?;
                    }

                    writeln!(out, "    @JsonProperty({:?})", field.json_name)?;
                    writeln!(out, "    private {} {};", field.type_, field.name)?;
                    writeln!(out)?;
                }
                writeln!(out, "    public {}() {{", name)?;
                writeln!(out, "    }}")?;

                for field in &fields {
                    writeln!(out)?;

                    write!(
                        out,
                        "{}",
                        description_with_message(
                            &field.metadata,
                            1,
                            &format!("Getter for {}.<p>", field.name)
                        )
                    )?;
                    writeln!(
                        out,
                        "    public {} get{}() {{",
                        field.type_,
                        capitalize_first_letter(&field.name)
                    )?;
                    writeln!(out, "        return {};", field.name)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                    write!(
                        out,
                        "{}",
                        description_with_message(
                            &field.metadata,
                            1,
                            &format!("Setter for {}.<p>", field.name)
                        )
                    )?;
                    writeln!(
                        out,
                        "    public void set{}({} {}) {{",
                        capitalize_first_letter(&field.name),
                        field.type_,
                        field.name
                    )?;
                    writeln!(out, "        this.{0} = {0};", field.name)?;
                    writeln!(out, "    }}")?;
                }

                writeln!(out, "}}")?;

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

                write!(out, "{}", description(&metadata, 0))?;
                writeln!(
                    out,
                    "@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = {:?})",
                    tag_json_name
                )?;
                writeln!(out, "@JsonSubTypes({{")?;
                for variant in variants {
                    writeln!(
                        out,
                        "    @JsonSubTypes.Type(name = {:?}, value = {}.class),",
                        variant.tag_value, variant.type_name
                    )?;
                }
                writeln!(out, "}})")?;
                writeln!(out, "public abstract class {} {{", name)?;
                writeln!(out, "}}")?;

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

                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "@JsonSerialize")?;
                if has_additional {
                    writeln!(out, "@JsonIgnoreProperties(ignoreUnknown = true)")?;
                }
                writeln!(out, "public class {} extends {} {{", name, parent_name)?;
                for field in &fields {
                    if field.optional {
                        writeln!(out, "    @JsonInclude(JsonInclude.Include.NON_NULL)")?;
                    }

                    writeln!(out, "    @JsonProperty({:?})", field.json_name)?;
                    writeln!(out, "    private {} {};", field.type_, field.name)?;
                    writeln!(out)?;
                }
                writeln!(out, "    public {}() {{", name)?;
                writeln!(out, "    }}")?;

                for field in &fields {
                    writeln!(out)?;

                    write!(
                        out,
                        "{}",
                        description_with_message(
                            &field.metadata,
                            1,
                            &format!("Getter for {}.<p>", field.name)
                        )
                    )?;
                    writeln!(
                        out,
                        "    public {} get{}() {{",
                        field.type_,
                        capitalize_first_letter(&field.name)
                    )?;
                    writeln!(out, "        return {};", field.name)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                    write!(
                        out,
                        "{}",
                        description_with_message(
                            &field.metadata,
                            1,
                            &format!("Setter for {}.<p>", field.name)
                        )
                    )?;
                    writeln!(
                        out,
                        "    public void set{}({} {}) {{",
                        capitalize_first_letter(&field.name),
                        field.type_,
                        field.name
                    )?;
                    writeln!(out, "        this.{0} = {0};", field.name)?;
                    writeln!(out, "    }}")?;
                }

                writeln!(out, "}}")?;

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
    version: &'a str,
    package: &'a str,
}

#[derive(Template)]
#[template(path = "UnsignedShort")]
struct UnsignedShortTemplate<'a> {
    version: &'a str,
    package: &'a str,
}

#[derive(Template)]
#[template(path = "UnsignedInteger")]
struct UnsignedIntegerTemplate<'a> {
    version: &'a str,
    package: &'a str,
}

fn description(metadata: &BTreeMap<String, Value>, indent: usize) -> String {
    doc(indent, jtd_codegen::target::metadata::description(metadata))
}

fn description_with_message(
    metadata: &BTreeMap<String, Value>,
    indent: usize,
    message: &str,
) -> String {
    doc(
        indent,
        &format!(
            "{}\n{}",
            message,
            jtd_codegen::target::metadata::description(metadata)
        ),
    )
}

fn capitalize_first_letter(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    head.to_uppercase() + tail
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
    jtd_codegen::target::fmt::comment_block(
        &format!("{}/**", prefix),
        &format!("{} * ", prefix),
        &format!("{} */", prefix),
        s,
    )
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("com.example".into()));
    }
}
