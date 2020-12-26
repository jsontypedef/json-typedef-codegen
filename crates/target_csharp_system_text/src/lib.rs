use askama::Template;
use jtd_codegen::target::inflect::*;
use jtd_codegen::target::*;
use jtd_codegen::Result;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

// todo: use keyword-avoiding inflectors
lazy_static! {
    static ref TYPE_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(CombiningInflector::new(Case::PascalCase));
    static ref FIELD_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::PascalCase));
    static ref ENUM_VARIANT_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::PascalCase));
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

    fn file_partitioning() -> FilePartitioning {
        FilePartitioning::FilePerType("cs".into())
    }

    fn enum_strategy() -> EnumStrategy {
        EnumStrategy::Modularized
    }

    fn name_type(name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(name_parts: &[String]) -> String {
        ENUM_VARIANT_NAMING_CONVENTION.inflect(name_parts)
    }

    fn booleans_are_nullable() -> bool {
        false
    }

    fn strings_are_nullable() -> bool {
        true
    }

    fn timestamps_are_nullable() -> bool {
        false
    }

    fn arrays_are_nullable() -> bool {
        true
    }

    fn aliases_are_nullable() -> bool {
        true
    }

    fn enums_are_nullable() -> bool {
        false
    }

    fn structs_are_nullable() -> bool {
        true
    }

    fn discriminators_are_nullable() -> bool {
        true
    }

    fn boolean(&self, _state: &mut Self::FileState) -> String {
        format!("bool")
    }

    fn string(&self, _state: &mut Self::FileState) -> String {
        format!("string")
    }

    fn timestamp(&self, state: &mut Self::FileState) -> String {
        state.imports.insert("System".into());
        format!("DateTimeOffset")
    }

    fn nullable_of(&self, _state: &mut Self::FileState, type_: String) -> String {
        format!("{}?", type_)
    }

    fn array_of(&self, state: &mut Self::FileState, type_: String) -> String {
        state.imports.insert("System.Collections.Generic".into());
        format!("IList<{}>", type_)
    }

    fn write_preamble(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()> {
        for namespace in &state.imports {
            writeln!(out, "using {};", namespace)?;
        }

        Ok(())
    }

    fn write_alias(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String> {
        state.imports.insert("System".into());

        state.imports.insert("System.Text.Json".into());

        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(
            out,
            "{}",
            AliasTemplate {
                namespace: &self.namespace,
                alias: &alias,
            }
            .render()
            .unwrap()
        )?;

        Ok(alias.name)
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String> {
        state.imports.insert("System".into());

        state.imports.insert("System.Text.Json".into());

        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(
            out,
            "{}",
            EnumTemplate {
                namespace: &self.namespace,
                enum_: &enum_,
            }
            .render()
            .unwrap()
        )?;

        Ok(enum_.name)
    }

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct,
    ) -> Result<String> {
        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(
            out,
            "{}",
            StructTemplate {
                namespace: &self.namespace,
                struct_: &struct_,
            }
            .render()
            .unwrap()
        )?;

        Ok(struct_.name)
    }

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator_variant: DiscriminatorVariant,
    ) -> Result<String> {
        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(
            out,
            "{}",
            DiscriminatorVariantTemplate {
                namespace: &self.namespace,
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
        state.imports.insert("System".into());
        state.imports.insert("System.Text.Json".into());
        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(
            out,
            "{}",
            DiscriminatorTemplate {
                namespace: &self.namespace,
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
#[template(path = "alias")]
struct AliasTemplate<'a> {
    namespace: &'a str,
    alias: &'a Alias,
}

#[derive(Template)]
#[template(path = "enum")]
struct EnumTemplate<'a> {
    namespace: &'a str,
    enum_: &'a Enum,
}

#[derive(Template)]
#[template(path = "struct")]
struct StructTemplate<'a> {
    namespace: &'a str,
    struct_: &'a Struct,
}

#[derive(Template)]
#[template(path = "discriminator_variant")]
struct DiscriminatorVariantTemplate<'a> {
    namespace: &'a str,
    discriminator_variant: &'a DiscriminatorVariant,
}

#[derive(Template)]
#[template(path = "discriminator")]
struct DiscriminatorTemplate<'a> {
    namespace: &'a str,
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
