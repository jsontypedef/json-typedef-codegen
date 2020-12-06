use crate::target::*;
use crate::Result;
use jtd::form::{self, TypeValue};
use jtd::{Form, Schema};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::namespace::Namespace;

mod ast {
    use crate::Target;
    use jtd::form::{self, TypeValue};
    use jtd::{Form, Schema};
    use std::collections::BTreeMap;
    use teeter_inflector::string::singularize::to_singular;

    #[derive(Debug)]
    pub enum Ast {
        Ref(String),
        Boolean,
        String,
        ElementsOf(Box<Ast>),
        Alias(Alias),
        Enum(Enum),
        Struct(Struct),
    }

    #[derive(Debug)]
    pub struct Alias {
        pub name: String,
        pub description: String,
        pub type_: Box<Ast>,
    }

    #[derive(Debug)]
    pub struct Enum {
        pub name: String,
        pub description: String,
        pub variants: Vec<EnumVariant>,
    }

    #[derive(Debug)]
    pub struct EnumVariant {
        pub name: String,
        pub description: String,
        pub json_value: String,
    }

    #[derive(Debug)]
    pub struct Struct {
        pub name: String,
        pub description: String,
        pub has_additional: bool,
        pub fields: Vec<StructField>,
    }

    #[derive(Debug)]
    pub struct StructField {
        pub name: String,
        pub json_name: String,
        pub description: String,
        pub optional: bool,
        pub type_: Ast,
    }

    pub fn from_schema<T: Target>(
        target: &T,
        root_name: String,
        schema: &Schema,
    ) -> (Ast, BTreeMap<String, Ast>) {
        let root = from_schema_top_level(target, root_name, schema);
        let definitions = schema
            .definitions
            .iter()
            .map(|(name, sub_schema)| {
                (
                    name.clone(),
                    from_schema_top_level(target, name.clone(), sub_schema),
                )
            })
            .collect();

        (root, definitions)
    }

    fn from_schema_top_level<T: Target>(target: &T, name: String, schema: &Schema) -> Ast {
        let ast = _from_schema(target, &mut vec![name.clone()], schema);
        match ast {
            Ast::Alias(_) | Ast::Enum(_) | Ast::Struct(_) => ast,
            _ => Ast::Alias(Alias {
                name: target.name_type(&[name]),
                description: "".into(),
                type_: Box::new(ast),
            }),
        }
    }

    fn _from_schema<T: Target>(target: &T, path: &mut Vec<String>, schema: &Schema) -> Ast {
        match schema.form {
            Form::Ref(form::Ref { ref definition, .. }) => Ast::Ref(definition.clone()),
            Form::Type(form::Type { ref type_value, .. }) => match type_value {
                TypeValue::Boolean => Ast::Boolean,
                TypeValue::String => Ast::String,
                _ => todo!(),
            },
            Form::Enum(form::Enum { ref values, .. }) => {
                let mut variants = vec![];
                for value in values {
                    path.push(value.into());
                    variants.push(EnumVariant {
                        name: target.name_enum_variant(path),
                        description: "".into(),
                        json_value: value.into(),
                    });
                    path.pop();
                }

                let name = target.name_type(path);
                Ast::Enum(Enum {
                    name,
                    description: "".into(),
                    variants,
                })
            }
            Form::Elements(form::Elements {
                ref schema,
                nullable,
            }) => {
                // singularize the last path segment, because it turns out it was
                // referring to a list of things, and thus we presume the name may
                // be in the plural.
                let last = path.pop().expect("todo: top-level elements");
                path.push(to_singular(&last));

                Ast::ElementsOf(Box::new(_from_schema(target, path, schema)))
            }
            Form::Properties(form::Properties {
                ref required,
                ref optional,
                additional,
                ..
            }) => {
                let struct_name = target.name_type(path);

                let mut fields = Vec::new();
                for (name, sub_schema) in required {
                    // Determine the field's name.
                    path.push(name.clone());
                    let field_name = target.name_field(path);
                    path.pop();

                    // Generate an expr representing sub_schema
                    path.push(name.clone());
                    let field_ast = _from_schema(target, path, sub_schema);
                    path.pop();

                    // Add the expr to the set of fields for the struct
                    fields.push(StructField {
                        name: field_name,
                        json_name: name.clone(),
                        description: "".into(),
                        optional: false,
                        type_: field_ast,
                    });
                }

                Ast::Struct(Struct {
                    name: struct_name,
                    description: "".into(),
                    has_additional: additional,
                    fields,
                })
            }
            _ => todo!(),
        }
    }
}

pub fn codegen<T: Target>(
    target: &T,
    root_name: String,
    schema: &Schema,
    out_dir: &Path,
) -> Result<Expr<T::ExprMeta>> {
    let (root, definitions) = ast::from_schema(target, root_name, schema);

    let defs = definitions
        .iter()
        .map(|(name, ast)| {
            (
                name.clone(),
                match ast {
                    ast::Ast::Struct(s) => s.name.clone(),
                    ast::Ast::Alias(a) => a.name.clone(),
                    _ => unreachable!(),
                },
            )
        })
        .collect();

    let global_state = GlobalState {
        file_partitioning: target.file_partitioning(),
        enum_strategy: target.enum_strategy(),
        target,
        defs: &defs,
        out_dir,
    };

    let mut file_state = FileState {
        buf: Vec::new(),
        target_state: T::FileState::default(),
    };

    for (_, def_ast) in definitions {
        _codegen(&global_state, &mut file_state, def_ast)?;
    }

    let root_expr = _codegen(&global_state, &mut file_state, root)?;

    // If we are doing single-file file partitioning, then no schema will ever
    // write itself out to a file. We will need to flush the single file out
    // here, now that all code has been generated.
    if let FilePartitioning::SingleFile(_) = global_state.file_partitioning {
        write_out_file(&global_state, &mut file_state, &root_expr)?;
    }

    Ok(root_expr)
}

struct GlobalState<'a, T: Target> {
    file_partitioning: FilePartitioning,
    enum_strategy: EnumStrategy,
    target: &'a T,
    defs: &'a BTreeMap<String, String>,
    out_dir: &'a Path,
}

struct FileState<T: Target> {
    buf: Vec<u8>,
    target_state: T::FileState,
}

fn _codegen<'a, T: Target>(
    global: &GlobalState<'a, T>,
    file: &mut FileState<T>,
    ast_: ast::Ast,
) -> Result<Expr<T::ExprMeta>> {
    Ok(match ast_ {
        ast::Ast::Ref(def) => Expr {
            expr: global.defs[&def].clone(),
            meta: T::ExprMeta::universally_usable(),
        },
        ast::Ast::Boolean => global.target.boolean(&mut file.target_state),
        ast::Ast::String => global.target.string(&mut file.target_state),
        ast::Ast::ElementsOf(sub_ast) => {
            let sub_expr = _codegen(global, file, *sub_ast)?;
            global.target.elements_of(&mut file.target_state, sub_expr)
        }
        ast::Ast::Alias(alias) => with_subfile_state(global, Some(file), |file| {
            let sub_expr = _codegen(global, file, *alias.type_)?;
            global.target.write_alias(
                &mut file.target_state,
                &mut file.buf,
                Alias {
                    name: alias.name.clone(),
                    description: alias.description,
                    type_: sub_expr,
                },
            )
        })?,
        ast::Ast::Enum(enum_) => with_subfile_state(global, Some(file), |file| {
            let mut variant_names = Namespace::new();
            let mut variants = vec![];

            for variant in enum_.variants {
                let name = variant_names.get(variant.name);

                variants.push(EnumVariant {
                    name,
                    description: variant.description,
                    json_value: variant.json_value,
                })
            }

            if let EnumStrategy::Unmodularized = global.enum_strategy {
                for variant in &variants {
                    global.target.write_enum_variant(
                        &mut file.target_state,
                        &mut file.buf,
                        variant.clone(),
                    )?;
                }
            }

            global.target.write_enum(
                &mut file.target_state,
                &mut file.buf,
                Enum {
                    name: enum_.name,
                    description: enum_.description,
                    variants: variants,
                },
            )
        })?,
        ast::Ast::Struct(struct_) => with_subfile_state(global, Some(file), |file| {
            let mut field_names = Namespace::new();
            let mut fields = Vec::new();

            for field in struct_.fields {
                let name = field_names.get(field.name);

                fields.push(StructField {
                    name: name,
                    json_name: field.json_name,
                    description: "".into(),
                    optional: field.optional,
                    type_: _codegen(global, file, field.type_)?,
                });
            }

            global.target.write_struct(
                &mut file.target_state,
                &mut file.buf,
                Struct {
                    name: struct_.name.clone(),
                    description: struct_.description,
                    has_additional: struct_.has_additional,
                    fields,
                },
            )
        })?,
    })
}

fn with_subfile_state<'a, T: Target, F: FnOnce(&mut FileState<T>) -> Result<Expr<T::ExprMeta>>>(
    global: &GlobalState<'a, T>,
    file_state: Option<&mut FileState<T>>,
    f: F,
) -> Result<Expr<T::ExprMeta>> {
    let mut default_file_state = FileState {
        buf: Vec::new(),
        target_state: T::FileState::default(),
    };

    let (mut subfile_state, should_write_out) = match global.file_partitioning {
        FilePartitioning::FilePerType(_) => (&mut default_file_state, true),
        FilePartitioning::SingleFile(_) => {
            let should_write_out = file_state.is_none();
            (
                file_state.unwrap_or(&mut default_file_state),
                should_write_out,
            )
        }
    };

    let expr = f(subfile_state)?;

    if should_write_out {
        write_out_file(global, &mut subfile_state, &expr)?;
        // let out_file_name = match global.file_partitioning {
        //     FilePartitioning::FilePerType(ref extension) => {
        //         Path::new(&expr.expr).with_extension(extension)
        //     }
        //     FilePartitioning::SingleFile(ref file_name) => Path::new(file_name).to_path_buf(),
        // };

        // dbg!(&out_file_name);

        // let mut out_file = File::create(global.out_dir.join(out_file_name))?;

        // global
        //     .target
        //     .write_preamble(&mut subfile_state.target_state, &mut out_file)?;
        // out_file.write_all(&subfile_state.buf)?;
    }

    Ok(expr)
}

fn write_out_file<'a, T: Target>(
    global: &GlobalState<'a, T>,
    file: &mut FileState<T>,
    expr: &Expr<T::ExprMeta>,
) -> Result<()> {
    let out_file_name = match global.file_partitioning {
        FilePartitioning::FilePerType(ref extension) => {
            Path::new(&expr.expr).with_extension(extension)
        }
        FilePartitioning::SingleFile(ref file_name) => Path::new(file_name).to_path_buf(),
    };

    let mut out_file = File::create(global.out_dir.join(out_file_name))?;

    global
        .target
        .write_preamble(&mut file.target_state, &mut out_file)?;
    out_file.write_all(&file.buf)?;

    Ok(())
}
