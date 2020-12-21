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
    type ExprMeta = ExprMeta;

    fn file_partitioning(&self) -> FilePartitioning {
        // todo: make sure this is a valid file name
        FilePartitioning::SingleFile(format!("{}.go", self.package))
    }

    fn enum_strategy(&self) -> EnumStrategy {
        EnumStrategy::Unmodularized
    }

    fn name_type(&self, name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(&self, name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(&self, name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn booleans_are_nullable(&self) -> bool {
        false
    }

    fn strings_are_nullable(&self) -> bool {
        false
    }

    fn timestamps_are_nullable(&self) -> bool {
        false
    }

    fn arrays_are_nullable(&self) -> bool {
        true
    }

    fn aliases_are_nullable(&self) -> bool {
        false
    }

    fn enums_are_nullable(&self) -> bool {
        false
    }

    fn structs_are_nullable(&self) -> bool {
        false
    }

    fn discriminators_are_nullable(&self) -> bool {
        false
    }

    fn boolean(&self, state: &mut Self::FileState) -> Expr<ExprMeta> {
        Expr {
            expr: format!("bool"),
            meta: ExprMeta { nullable: false },
        }
    }

    fn string(&self, state: &mut Self::FileState) -> Expr<ExprMeta> {
        Expr {
            expr: format!("string"),
            meta: ExprMeta { nullable: false },
        }
    }

    fn timestamp(&self, state: &mut Self::FileState) -> Expr<ExprMeta> {
        state.imports.insert("time".into());

        Expr {
            expr: format!("time.Time"),
            meta: ExprMeta { nullable: false },
        }
    }

    fn nullable_of(&self, state: &mut Self::FileState, expr: Expr<ExprMeta>) -> Expr<ExprMeta> {
        Expr {
            expr: format!("*{}", expr.expr),
            meta: ExprMeta { nullable: true },
        }
    }

    fn array_of(&self, state: &mut Self::FileState, expr: Expr<ExprMeta>) -> Expr<ExprMeta> {
        Expr {
            expr: format!("[]{}", expr.expr),
            meta: ExprMeta { nullable: true },
        }
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
        alias: Alias<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
        writeln!(out, "type {} = {}", alias.name, alias.type_.expr)?;
        Ok(Expr {
            expr: alias.name,
            meta: alias.type_.meta,
        })
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<Expr<ExprMeta>> {
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

        Ok(Expr {
            expr: enum_.name,
            meta: ExprMeta { nullable: false },
        })
    }

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
        writeln!(out, "type {} struct {{", struct_.name)?;
        for field in struct_.fields {
            writeln!(
                out,
                "\t{} {} `json:{:?}`",
                field.name, field.type_.expr, field.json_name
            )?;
        }
        writeln!(out, "}}")?;

        Ok(Expr {
            expr: struct_.name,
            meta: ExprMeta { nullable: false },
        })
    }

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        variant: DiscriminatorVariant<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
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
                field.name, field.type_.expr, field.json_name
            )?;
        }
        writeln!(out, "}}")?;

        Ok(Expr {
            expr: variant.name,
            meta: ExprMeta { nullable: true },
        })
    }

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
        state.imports.insert("encoding/json".into());
        state.imports.insert("fmt".into());

        writeln!(out, "type {} struct {{", discriminator.name)?;

        writeln!(out, "\t{} string", discriminator.tag_name)?;
        for (_tag_value, variant) in &discriminator.variants {
            writeln!(out, "\t{} {}", variant.expr, variant.expr)?;
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
                discriminator.tag_json_name, variant.expr, discriminator.tag_name, variant.expr
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
                variant.expr
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

        Ok(Expr {
            expr: discriminator.name,
            meta: ExprMeta { nullable: true },
        })
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeSet<String>,
}

#[derive(PartialEq, Clone)]
pub struct ExprMeta {
    nullable: bool,
}

impl jtd_codegen::ExprMeta for ExprMeta {
    fn universally_usable() -> Self {
        Self { nullable: true }
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("jtd_codegen_e2e".into()));
    }
}
