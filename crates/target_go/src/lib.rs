use jtd_codegen::*;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

// todo: use keyword-avoiding inflectors
lazy_static! {
    // todo: more of an "item" naming convention, containing both types and
    // consts. What is the proper name for this, per the Go spec?
    static ref TYPE_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(CombiningInflector::new(Case::PascalCase));
    static ref FIELD_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::PascalCase));
}

pub struct Target {
    package: String,
}

impl Target {
    pub fn new(package: String) -> Self {
        Self { package }
    }
}

impl jtd_codegen::Target for Target {
    type FileState = FileState;

    fn file_partitioning() -> FilePartitioning {
        FilePartitioning::SingleFile("index.go".into())
    }

    fn enum_strategy() -> EnumStrategy {
        EnumStrategy::Unmodularized
    }

    fn name_type(name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
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

    fn boolean(&self, state: &mut Self::FileState) -> String {
        format!("bool")
    }

    fn string(&self, state: &mut Self::FileState) -> String {
        format!("string")
    }

    fn timestamp(&self, state: &mut Self::FileState) -> String {
        state.imports.insert("time".into());
        "time.Time".into()
    }

    fn nullable_of(&self, state: &mut Self::FileState, type_: String) -> String {
        format!("*{}", type_)
    }

    fn array_of(&self, state: &mut Self::FileState, type_: String) -> String {
        format!("[]{}", type_)
    }

    fn write_preamble(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()> {
        writeln!(out, "package {}", self.package)?;

        for package in &state.imports {
            writeln!(out, "import {:?}", package)?;
        }

        Ok(())
    }

    fn write_alias(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String> {
        writeln!(out, "type {} = {}", alias.name, alias.type_)?;
        Ok(alias.name)
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String> {
        writeln!(out, "type {} string", enum_.name)?;

        writeln!(out, "const (")?;
        for variant in enum_.variants {
            writeln!(
                out,
                "\t{} {} = {:?}",
                variant.name, enum_.name, variant.json_value
            )?;
        }
        writeln!(out, ")")?;

        Ok(enum_.name)
    }

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct,
    ) -> Result<String> {
        writeln!(out, "type {} struct {{", struct_.name)?;
        for field in struct_.fields {
            writeln!(
                out,
                "\t{} {} `json:{:?}`",
                field.name, field.type_, field.json_name
            )?;
        }
        writeln!(out, "}}")?;

        Ok(struct_.name)
    }

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        variant: DiscriminatorVariant,
    ) -> Result<String> {
        writeln!(out, "type {} struct {{", variant.name)?;
        writeln!(
            out,
            "\t{} string `json:{:?}`",
            variant.tag_name, variant.tag_json_name
        )?;
        for field in variant.fields {
            writeln!(
                out,
                "\t{} {} `json:{:?}`",
                field.name, field.type_, field.json_name
            )?;
        }
        writeln!(out, "}}")?;

        Ok(variant.name)
    }

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator,
    ) -> Result<String> {
        state.imports.insert("encoding/json".into());
        state.imports.insert("fmt".into());

        writeln!(out, "type {} struct {{", discriminator.name)?;

        writeln!(out, "\t{} string", discriminator.tag_name)?;
        for (_tag_value, variant) in &discriminator.variants {
            writeln!(out, "\t{} {}", variant, variant)?;
        }

        writeln!(out, "}}")?;

        writeln!(
            out,
            "func (v {}) MarshalJSON() ([]byte, error) {{",
            discriminator.name
        )?;
        writeln!(out, "\tswitch (v.{}) {{", discriminator.tag_name)?;
        for (tag_value, variant) in &discriminator.variants {
            writeln!(out, "\tcase {:?}:", tag_value)?;
            writeln!(
                out,
                "\t\treturn json.Marshal(struct {{ T string `json:{:?}`; {} }}{{ v.{}, v.{} }})",
                discriminator.tag_json_name, variant, discriminator.tag_name, variant
            )?;
        }
        writeln!(out, "\t}}")?;
        writeln!(
            out,
            "\treturn nil, fmt.Errorf(\"bad {} value: %s\", v.{})",
            discriminator.tag_name, discriminator.tag_name
        )?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "func (v *{}) UnmarshalJSON(b []byte) error {{",
            discriminator.name
        )?;
        writeln!(
            out,
            "\tvar t struct {{ T string `json:{:?}` }}",
            discriminator.tag_json_name
        )?;
        writeln!(out, "\tif err := json.Unmarshal(b, &t); err != nil {{")?;
        writeln!(out, "\t\treturn err")?;
        writeln!(out, "\t}}")?;
        writeln!(out, "\tswitch t.T {{")?;
        for (tag_value, variant) in &discriminator.variants {
            writeln!(out, "\tcase {:?}:", tag_value)?;
            writeln!(
                out,
                "\t\tif err := json.Unmarshal(b, &v.{}); err != nil {{",
                variant
            )?;
            writeln!(out, "\t\t\treturn err")?;
            writeln!(out, "\t\t}}")?;
            writeln!(out, "\t\tv.{} = {:?}", discriminator.tag_name, tag_value)?;
            writeln!(out, "\t\treturn nil")?;
        }
        writeln!(out, "\t}}")?;
        writeln!(
            out,
            "\treturn fmt.Errorf(\"bad {} value: %s\", t.T)",
            discriminator.tag_name
        )?;
        writeln!(out, "}}")?;

        Ok(discriminator.name)
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("jtd_codegen_e2e".into()));
    }
}
