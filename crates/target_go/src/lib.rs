use askama::Template;
use jtd_codegen::target::inflect::{self, Inflector};
use jtd_codegen::target::*;
use jtd_codegen::Result;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref IDENT_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::PascalCase)
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::PascalCase)
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

    fn file_partitioning(&self) -> FilePartitioning {
        FilePartitioning::SingleFile(format!(
            "{}.go",
            inflect::KeywordAvoidingInflector::new(
                KEYWORDS.clone(),
                inflect::TailInflector::new(inflect::Case::SnakeCase)
            )
            .inflect(&[self.package.clone()])
        ))
    }

    fn enum_strategy(&self) -> EnumStrategy {
        EnumStrategy::Unmodularized
    }

    fn name_type(name_parts: &[String]) -> String {
        IDENT_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(name_parts: &[String]) -> String {
        IDENT_NAMING_CONVENTION.inflect(name_parts)
    }

    fn booleans_are_nullable() -> bool {
        false
    }

    fn strings_are_nullable() -> bool {
        false
    }

    fn timestamps_are_nullable() -> bool {
        false
    }

    fn arrays_are_nullable() -> bool {
        true
    }

    fn aliases_are_nullable() -> bool {
        false
    }

    fn enums_are_nullable() -> bool {
        false
    }

    fn structs_are_nullable() -> bool {
        false
    }

    fn discriminators_are_nullable() -> bool {
        false
    }

    fn boolean(&self, _state: &mut Self::FileState) -> String {
        format!("bool")
    }

    fn string(&self, _state: &mut Self::FileState) -> String {
        format!("string")
    }

    fn timestamp(&self, state: &mut Self::FileState) -> String {
        state.imports.insert("time".into());
        "time.Time".into()
    }

    fn nullable_of(&self, _state: &mut Self::FileState, type_: String) -> String {
        format!("*{}", type_)
    }

    fn array_of(&self, _state: &mut Self::FileState, type_: String) -> String {
        format!("[]{}", type_)
    }

    fn write_preamble(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()> {
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

        Ok(())
    }

    fn write_alias(
        &self,
        _state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String> {
        writeln!(out, "type {} = {}", alias.name, alias.type_)?;
        Ok(alias.name)
    }

    fn write_enum(
        &self,
        _state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String> {
        writeln!(out, "{}", EnumTemplate { enum_: &enum_ }.render().unwrap())?;

        Ok(enum_.name)
    }

    fn write_struct(
        &self,
        _state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct,
    ) -> Result<String> {
        writeln!(
            out,
            "{}",
            StructTemplate { struct_: &struct_ }.render().unwrap()
        )?;

        Ok(struct_.name)
    }

    fn write_discriminator_variant(
        &self,
        _state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator_variant: DiscriminatorVariant,
    ) -> Result<String> {
        writeln!(
            out,
            "{}",
            DiscriminatorVariantTemplate {
                discriminator_variant: &discriminator_variant,
            }
            .render()
            .unwrap()
        )?;

        Ok(discriminator_variant.name)
    }

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator,
    ) -> Result<String> {
        state.imports.insert("encoding/json".into());
        state.imports.insert("fmt".into());

        writeln!(
            out,
            "{}",
            DiscriminatorTemplate {
                discriminator: &discriminator,
            }
            .render()
            .unwrap()
        )?;

        Ok(discriminator.name)
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeSet<String>,
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
    alias: &'a Alias,
}

#[derive(Template)]
#[template(path = "enum")]
struct EnumTemplate<'a> {
    enum_: &'a Enum,
}

#[derive(Template)]
#[template(path = "struct")]
struct StructTemplate<'a> {
    struct_: &'a Struct,
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    discriminator_variant: &'a DiscriminatorVariant,
}

#[derive(Template)]
#[template(path = "discriminator")]
struct DiscriminatorTemplate<'a> {
    discriminator: &'a Discriminator,
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
        jtd_codegen::target::fmt::comment_block("", &format!("{}// ", prefix), "", s)
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("jtd_codegen_e2e".into()));
    }
}
