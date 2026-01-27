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
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case_with_initialisms(INITIALISMS.clone())),
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::new(
                inflect::CaseCapitalization::None,
                inflect::CaseCapitalization::Initial,
                None,
                INITIALISMS.clone(),
            )),
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case_with_initialisms(INITIALISMS.clone())),
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
            file_partitioning: target::FilePartitioningStrategy::SingleFile("jtd.hpp".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Unmodularized,
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
        if let Some(s) = metadata.get("cppType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "std::any".into(),
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => { state.includes.insert("<cstdint>".into()); "int8_t".into() },
            target::Expr::Uint8 => { state.includes.insert("<cstdint>".into()); "uint8_t".into() },
            target::Expr::Int16 => { state.includes.insert("<cstdint>".into()); "int16_t".into() },
            target::Expr::Uint16 => { state.includes.insert("<cstdint>".into()); "uint16_t".into() },
            target::Expr::Int32 => { state.includes.insert("<cstdint>".into()); "int32_t".into() },
            target::Expr::Uint32 => { state.includes.insert("<cstdint>".into()); "uint32_t".into() },
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "double".into(),
            target::Expr::String => { state.includes.insert("<string>".into()); "std::string".into() },
            target::Expr::Timestamp => { state.includes.insert("<string>".into()); "std::string".into() },
            target::Expr::ArrayOf(sub_expr) => {
                state.includes.insert("<vector>".into());
                format!("std::vector<{}>", sub_expr)
            },
            target::Expr::DictOf(sub_expr) => {
                state.includes.insert("<map>".into());
                state.includes.insert("<string>".into());
                format!("std::map<std::string, {}>", sub_expr)
            },
            target::Expr::NullableOf(sub_expr) => {
                state.includes.insert("<optional>".into());
                format!("std::optional<{}>", sub_expr)
            },
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut FileState,
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Auxiliary { .. } => None,
            target::Item::Preamble => {
                None
            }
            target::Item::Postamble => {
                writeln!(out, "// Code generated by jtd-codegen for C++. DO NOT EDIT.")?;
                writeln!(out, "#pragma once")?;
                writeln!(out)?;

                for inc in &state.includes {
                    writeln!(out, "#include {}", inc)?;
                }

                if state.includes.contains("<any>") {
                    // Placeholder for future logic if std::any is actually used
                }

                writeln!(out)?;
                writeln!(out, "namespace {} {{", self.namespace)?;
                writeln!(out)?;

                for decl in &state.forward_decls {
                    writeln!(out, "{}", decl)?;
                }
                writeln!(out)?;

                for def in &state.definitions {
                    writeln!(out, "{}", def)?;
                }

                writeln!(out, "}} // namespace {}", self.namespace)?;
                None
            },
            target::Item::Alias { metadata, name, type_ } => {
                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("using {} = {};", name, type_));
                state.definitions.push(s);
                None
            }
            target::Item::Enum { metadata, name, members } => {
                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("enum class {} {{\n", name));
                for member in members {
                    s.push_str(&enum_variant_description(&metadata, 1, &member.json_value));
                    s.push_str(&format!("\t{},\n", member.name));
                }
                s.push_str("};");
                state.definitions.push(s);
                None
            }
            target::Item::Struct { metadata, name, fields, .. } => {
                state.forward_decls.insert(format!("struct {};", name));

                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("struct {} {{\n", name));
                for field in fields {
                    s.push_str(&description(&field.metadata, 1));
                    s.push_str(&format!("\t{} {};\n", field.type_, field.name));
                }
                s.push_str("};");
                state.definitions.push(s);
                None
            }
            target::Item::Discriminator { metadata, name, tag_field_name, variants: _, .. } => {
                state.forward_decls.insert(format!("struct {};", name));

                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("struct {} {{\n", name));
                s.push_str(&format!("\tstd::string {};\n", tag_field_name));
                s.push_str("};");
                state.definitions.push(s);
                None
            }
            target::Item::DiscriminatorVariant { metadata, name, fields, .. } => {
                state.forward_decls.insert(format!("struct {};", name));

                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("struct {} {{\n", name));
                for field in fields {
                     s.push_str(&description(&field.metadata, 1));
                     s.push_str(&format!("\t{} {};\n", field.type_, field.name));
                }
                 s.push_str("};");
                state.definitions.push(s);
                None
            }
        })
    }
}

pub struct FileState {
    includes: BTreeSet<String>,
    forward_decls: BTreeSet<String>,
    definitions: Vec<String>,
}

impl Default for FileState {
    fn default() -> Self {
        Self {
            includes: BTreeSet::new(),
            forward_decls: BTreeSet::new(),
            definitions: Vec::new(),
        }
    }
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
    let prefix = "\t".repeat(ident);
    jtd_codegen::target::fmt::comment_block("", &format!("{}// ", prefix), "", s)
}
