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

    fn boolean(&self, state: &mut Self::FileState) -> Expr<ExprMeta> {
        Expr {
            expr: format!("bool"),
            meta: ExprMeta { nullable: false },
        }
    }

    fn string(&self, state: &mut Self::FileState) -> Expr<ExprMeta> {
        Expr {
            expr: format!("string"),
            meta: ExprMeta { nullable: true },
        }
    }

    fn nullable_of(&self, state: &mut Self::FileState, expr: Expr<ExprMeta>) -> Expr<ExprMeta> {
        // It's already nullable, no need to do it again.
        if expr.meta.nullable {
            return expr;
        }

        Expr {
            expr: format!("*{}", expr.expr),
            meta: ExprMeta { nullable: true },
        }
    }

    fn elements_of(&self, state: &mut Self::FileState, expr: Expr<ExprMeta>) -> Expr<ExprMeta> {
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
        // todo: deal with weirdness around what types are correct to alias vs
        // embed (vs other techniques?)
        writeln!(out, "type {} = {}", alias.name, alias.type_.expr)?;
        Ok(Expr {
            expr: alias.name,
            meta: alias.type_.meta,
        })
    }

    fn write_enum_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        variant: EnumVariant,
    ) -> Result<Expr<ExprMeta>> {
        writeln!(out, "const {} = {:?}", variant.name, variant.json_value)?;
        Ok(Expr {
            expr: variant.name,
            meta: ExprMeta { nullable: false },
        })
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<Expr<ExprMeta>> {
        writeln!(out, "type {} string", enum_.name)?;
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
    use super::Target;

    #[test]
    fn test_common_test_cases() {
        let target = Target::new("jtd_codegen_e2e".into());
        jtd_codegen_test::assert_common_test_cases(env!("CARGO_MANIFEST_DIR"), &target);
    }
}
