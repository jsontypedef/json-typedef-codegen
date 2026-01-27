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
            inflect::TailInflector::new(inflect::Case::snake_case()),
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::snake_case()),
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::screaming_snake_case()),
        ));
}

pub struct Target {
}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("jtd.h".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Unmodularized,
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
            timestamps_are_nullable: true,
            arrays_are_nullable: true,
            dicts_are_nullable: true,
            aliases_are_nullable: false,
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
        if let Some(s) = metadata.get("cType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "void *".into(),
            target::Expr::Boolean => { state.includes.insert("<stdbool.h>".into()); "bool".into() },
            target::Expr::Int8 => { state.includes.insert("<stdint.h>".into()); "int8_t".into() },
            target::Expr::Uint8 => { state.includes.insert("<stdint.h>".into()); "uint8_t".into() },
            target::Expr::Int16 => { state.includes.insert("<stdint.h>".into()); "int16_t".into() },
            target::Expr::Uint16 => { state.includes.insert("<stdint.h>".into()); "uint16_t".into() },
            target::Expr::Int32 => { state.includes.insert("<stdint.h>".into()); "int32_t".into() },
            target::Expr::Uint32 => { state.includes.insert("<stdint.h>".into()); "uint32_t".into() },
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "double".into(),
            target::Expr::String => "char *".into(),
            target::Expr::Timestamp => "char *".into(),
            target::Expr::ArrayOf(sub_expr) => {
                state.includes.insert("<stddef.h>".into());
                let clean_sub = sub_expr.replace("*", "ptr").replace(" ", "");
                let type_name = format!("jtd_array_{}", clean_sub);

                if !state.generated_helper_types.contains(&type_name) {
                    state.generated_helper_types.insert(type_name.clone());
                    state.helper_types.push(format!("typedef struct {{ {} * data; size_t size; }} {};", sub_expr, type_name));
                }
                type_name
            },
            target::Expr::DictOf(sub_expr) => {
                 state.includes.insert("<stddef.h>".into());
                 let clean_sub = sub_expr.replace("*", "ptr").replace(" ", "");
                 let type_name = format!("jtd_dict_{}", clean_sub);

                if !state.generated_helper_types.contains(&type_name) {
                    state.generated_helper_types.insert(type_name.clone());
                    let entry_name = format!("{}_entry", type_name);
                    state.helper_types.push(format!("typedef struct {{ char * key; {} value; }} {};", sub_expr, entry_name));
                    state.helper_types.push(format!("typedef struct {{ {} * entries; size_t size; }} {};", entry_name, type_name));
                }
                type_name
            },
            target::Expr::NullableOf(sub_expr) => {
                if sub_expr.ends_with("*") {
                    sub_expr
                } else {
                    format!("{} *", sub_expr)
                }
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
                writeln!(out, "// Code generated by jtd-codegen for C. DO NOT EDIT.")?;
                writeln!(out, "#ifndef JTD_GENERATED_H")?;
                writeln!(out, "#define JTD_GENERATED_H")?;
                writeln!(out)?;

                for inc in &state.includes {
                    writeln!(out, "#include {}", inc)?;
                }
                writeln!(out)?;

                for decl in &state.forward_decls {
                    writeln!(out, "{}", decl)?;
                }
                writeln!(out)?;

                for helper in &state.helper_types {
                    writeln!(out, "{}", helper)?;
                }
                writeln!(out)?;

                for def in &state.definitions {
                     writeln!(out, "{}", def)?;
                }

                writeln!(out, "#endif // JTD_GENERATED_H")?;
                None
            },
            target::Item::Alias { metadata, name, type_ } => {
                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("typedef {} {};", type_, name));
                state.definitions.push(s);
                None
            }
            target::Item::Enum { metadata, name, members } => {
                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("typedef enum {{\n"));
                for (i, member) in members.iter().enumerate() {
                    s.push_str(&enum_variant_description(&metadata, 1, &member.json_value));
                    s.push_str(&format!("\t{}_{} = {},\n", name.to_uppercase(), member.name, i));
                }
                s.push_str(&format!("}} {};", name));
                state.definitions.push(s);
                None
            }
            target::Item::Struct { metadata, name, fields, .. } => {
                // Emitting forward declaration
                state.forward_decls.insert(format!("typedef struct {0} {0};", name));

                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("struct {} {{\n", name));
                for field in fields {
                     s.push_str(&description(&field.metadata, 1));
                     s.push_str(&format!("\t{} {};\n", field.type_, field.name));
                }
                s.push_str("};"); // End of struct definition
                state.definitions.push(s);
                None
            }
            target::Item::Discriminator { metadata, name, tag_field_name, variants: _, .. } => {
                state.forward_decls.insert(format!("typedef struct {0} {0};", name));

                let mut s = String::new();
                s.push_str(&description(&metadata, 0));
                s.push_str(&format!("struct {} {{\n", name));
                s.push_str(&format!("\tchar * {};\n", tag_field_name));
                s.push_str("};");
                state.definitions.push(s);
                None
            }
            target::Item::DiscriminatorVariant { metadata, name, fields, .. } => {
                state.forward_decls.insert(format!("typedef struct {0} {0};", name));

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
    generated_helper_types: BTreeSet<String>,
    helper_types: Vec<String>,
    forward_decls: BTreeSet<String>,
    definitions: Vec<String>,
}

impl Default for FileState {
    fn default() -> Self {
        Self {
            includes: BTreeSet::new(),
            generated_helper_types: BTreeSet::new(),
            helper_types: Vec::new(),
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
