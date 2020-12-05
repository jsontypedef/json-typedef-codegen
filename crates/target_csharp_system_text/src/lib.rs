use jtd_codegen::*;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

// todo: use keyword-avoiding inflectors
lazy_static! {
    static ref FILE_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> = Box::new(
        AppendingInflector::new(".cs".into(), TailInflector::new(Case::PascalCase))
    );
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

impl jtd_codegen::Target for Target {
    type FileState = FileState;
    type ExprMeta = ExprMeta;

    fn file_partitioning(&self) -> FilePartitioning {
        FilePartitioning::FilePerType("cs".into())
    }

    fn name_type(&self, name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(&self, name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(&self, name_parts: &[String]) -> String {
        ENUM_VARIANT_NAMING_CONVENTION.inflect(name_parts)
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
            expr: format!("{}?", expr.expr),
            meta: ExprMeta { nullable: true },
        }
    }

    fn elements_of(&self, state: &mut Self::FileState, expr: Expr<ExprMeta>) -> Expr<ExprMeta> {
        state.imports.insert("System.Collections.Generic".into());

        Expr {
            expr: format!("IList<{}>", expr.expr),
            meta: ExprMeta { nullable: true },
        }
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
        alias: Alias<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(out, "    public class {}", alias.name)?;
        writeln!(out, "    {{")?;
        writeln!(
            out,
            "        public Value {} {{ get; set; }}",
            alias.type_.expr
        )?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(Expr {
            expr: alias.name,
            meta: ExprMeta { nullable: true },
        })
    }

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct<ExprMeta>,
    ) -> Result<Expr<ExprMeta>> {
        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(out, "    public class {}", struct_.name)?;
        writeln!(out, "    {{")?;

        for field in struct_.fields {
            writeln!(out, "        [JsonPropertyName({:?})]", field.json_name)?;
            writeln!(
                out,
                "        public {} {} {{ get; set; }}",
                field.type_.expr, field.name
            )?;
        }

        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(Expr {
            expr: struct_.name,
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
    use super::Target;

    #[test]
    fn test_common_test_cases() {
        let target = Target::new("JtdCodegen.Demo".into());
        jtd_codegen_test::assert_common_test_cases(env!("CARGO_MANIFEST_DIR"), &target);
    }
}
