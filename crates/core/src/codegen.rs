use crate::namespace::Namespace;
use crate::target::*;
use crate::Result;
use jtd::form::{self, TypeValue};
use jtd::{Form, Schema};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

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
        Timestamp,
        ArrayOf(Box<Ast>),
        NullableOf(Box<Ast>),
        Alias(Alias),
        Enum(Enum),
        Struct(Struct),
        Discriminator(Discriminator),
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

    #[derive(Debug)]
    pub struct Discriminator {
        pub name: String,
        pub description: String,
        pub tag_name: String,
        pub tag_json_name: String,
        pub variants: BTreeMap<String, DiscriminatorVariant>, // key is tag value
    }

    #[derive(Debug)]
    pub struct DiscriminatorVariant {
        pub name: String,
        pub description: String,
        pub tag_name: String,
        pub tag_json_name: String,
        pub tag_json_value: String,
        pub fields: Vec<StructField>,
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
            Ast::Alias(_) | Ast::Enum(_) | Ast::Struct(_) | Ast::Discriminator(_) => ast,
            _ => Ast::Alias(Alias {
                name: target.name_type(&[name]),
                description: "".into(),
                type_: Box::new(ast),
            }),
        }
    }

    fn _from_schema<T: Target>(target: &T, path: &mut Vec<String>, schema: &Schema) -> Ast {
        match schema.form {
            Form::Ref(form::Ref {
                ref definition,
                nullable,
            }) => with_nullable(target, nullable, Ast::Ref(definition.clone())),
            Form::Type(form::Type {
                ref type_value,
                nullable,
            }) => with_nullable(
                target,
                nullable,
                match type_value {
                    TypeValue::Boolean => Ast::Boolean,
                    TypeValue::String => Ast::String,
                    TypeValue::Timestamp => Ast::Timestamp,
                    _ => todo!(),
                },
            ),
            Form::Enum(form::Enum {
                ref values,
                nullable,
            }) => {
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
                with_nullable(
                    target,
                    nullable,
                    Ast::Enum(Enum {
                        name,
                        description: "".into(),
                        variants,
                    }),
                )
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

                with_nullable(
                    target,
                    nullable,
                    Ast::ArrayOf(Box::new(_from_schema(target, path, schema))),
                )
            }
            Form::Properties(form::Properties {
                ref required,
                ref optional,
                additional,
                nullable,
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

                with_nullable(
                    target,
                    nullable,
                    Ast::Struct(Struct {
                        name: struct_name,
                        description: "".into(),
                        has_additional: additional,
                        fields,
                    }),
                )
            }
            Form::Discriminator(form::Discriminator {
                ref discriminator,
                ref mapping,
                nullable,
            }) => {
                let discriminator_name = target.name_type(path);

                path.push(discriminator.clone());
                let tag_name = target.name_field(path);
                path.pop();

                let mut variants = BTreeMap::new();
                for (tag_value, sub_schema) in mapping {
                    path.push(tag_value.clone());
                    let sub_expr = _from_schema(target, path, sub_schema);
                    path.pop();

                    // We know we are returning a struct here, because
                    // sub_schema must be of the properties form.
                    let struct_ = match sub_expr {
                        Ast::Struct(s) => s,
                        _ => unreachable!(),
                    };

                    variants.insert(
                        tag_value.clone(),
                        DiscriminatorVariant {
                            name: struct_.name,
                            description: struct_.description,
                            tag_name: tag_name.clone(),
                            tag_json_name: discriminator.clone(),
                            tag_json_value: tag_value.clone(),
                            fields: struct_.fields,
                        },
                    );
                }

                with_nullable(
                    target,
                    nullable,
                    Ast::Discriminator(Discriminator {
                        name: discriminator_name,
                        tag_name: tag_name.clone(),
                        tag_json_name: discriminator.clone(),
                        description: "".into(),
                        variants,
                    }),
                )
            }
            _ => todo!(),
        }
    }

    fn with_nullable<T: Target>(target: &T, nullable: bool, ast: Ast) -> Ast {
        // We need to wrap ast in NullableOf if the caller passed in nullable
        // and if ast isn't already nullable to begin with.
        let needs_nullable = nullable
            && match ast {
                Ast::Boolean => !target.booleans_are_nullable(),
                Ast::String => !target.strings_are_nullable(),
                Ast::Timestamp => !target.timestamps_are_nullable(),
                Ast::ArrayOf(_) => !target.arrays_are_nullable(),
                Ast::Alias(_) => !target.aliases_are_nullable(),
                Ast::Enum(_) => !target.enums_are_nullable(),
                Ast::Struct(_) => !target.structs_are_nullable(),
                Ast::Discriminator(_) => !target.discriminators_are_nullable(),
                Ast::Ref(_) => true,
                Ast::NullableOf(_) => false,
            };

        if needs_nullable {
            Ast::NullableOf(Box::new(ast))
        } else {
            ast
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
    dbg!(&root, &definitions);
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

    let mut global_state = GlobalState {
        file_partitioning: target.file_partitioning(),
        enum_strategy: target.enum_strategy(),
        names: Namespace::new(),
        target,
        defs: &defs,
        out_dir,
    };

    let mut file_state = FileState {
        buf: Vec::new(),
        target_state: T::FileState::default(),
    };

    for (_, def_ast) in definitions {
        _codegen(&mut global_state, &mut file_state, def_ast)?;
    }

    let root_expr = _codegen(&mut global_state, &mut file_state, root)?;

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
    names: Namespace,
    target: &'a T,
    defs: &'a BTreeMap<String, String>,
    out_dir: &'a Path,
}

struct FileState<T: Target> {
    buf: Vec<u8>,
    target_state: T::FileState,
}

fn _codegen<'a, T: Target>(
    global: &mut GlobalState<'a, T>,
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
        ast::Ast::Timestamp => global.target.timestamp(&mut file.target_state),
        ast::Ast::ArrayOf(sub_ast) => {
            let sub_expr = _codegen(global, file, *sub_ast)?;
            global.target.array_of(&mut file.target_state, sub_expr)
        }
        ast::Ast::NullableOf(sub_ast) => {
            let sub_expr = _codegen(global, file, *sub_ast)?;
            global.target.nullable_of(&mut file.target_state, sub_expr)
        }
        ast::Ast::Alias(alias) => with_subfile_state(global, Some(file), |global, file| {
            let sub_expr = _codegen(global, file, *alias.type_)?;
            global.target.write_alias(
                &mut file.target_state,
                &mut file.buf,
                Alias {
                    name: global.names.get(alias.name),
                    description: alias.description,
                    type_: sub_expr,
                },
            )
        })?,
        ast::Ast::Enum(enum_) => with_subfile_state(global, Some(file), |global, file| {
            let mut variant_names = Namespace::new();
            let mut variants = vec![];

            for variant in enum_.variants {
                let name = match global.enum_strategy {
                    EnumStrategy::Modularized => variant_names.get(variant.name),
                    EnumStrategy::Unmodularized => global.names.get(variant.name),
                };

                variants.push(EnumVariant {
                    name,
                    description: variant.description,
                    json_value: variant.json_value,
                })
            }

            global.target.write_enum(
                &mut file.target_state,
                &mut file.buf,
                Enum {
                    name: global.names.get(enum_.name),
                    description: enum_.description,
                    variants: variants,
                },
            )
        })?,
        ast::Ast::Struct(struct_) => with_subfile_state(global, Some(file), |global, file| {
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
                    name: global.names.get(struct_.name),
                    description: struct_.description,
                    has_additional: struct_.has_additional,
                    fields,
                },
            )
        })?,
        ast::Ast::Discriminator(discriminator) => {
            with_subfile_state(global, Some(file), |global, file| {
                let discriminator_name = global.names.get(discriminator.name);

                // Clone these as variables to avoid issues with partial moves
                // of discriminator.
                let tag_name = discriminator.tag_name.clone();
                let tag_json_name = discriminator.tag_json_name.clone();

                let mut variants = BTreeMap::new();
                for (tag_value, variant) in discriminator.variants {
                    variants.insert(
                        tag_value,
                        with_subfile_state(global, Some(file), |global, file| {
                            // todo: dedupe this logic with struct stuff above?
                            let mut field_names = Namespace::new();
                            let mut fields = Vec::new();

                            // Set aside a name for the discriminator tag,
                            // because it will probably be in the same namespace
                            // as the fields of the variant.
                            let tag_name = field_names.get(tag_name.clone());

                            for field in variant.fields {
                                let name = field_names.get(field.name);

                                fields.push(StructField {
                                    name: name,
                                    json_name: field.json_name,
                                    description: "".into(),
                                    optional: field.optional,
                                    type_: _codegen(global, file, field.type_)?,
                                });
                            }

                            global.target.write_discriminator_variant(
                                &mut file.target_state,
                                &mut file.buf,
                                DiscriminatorVariant {
                                    name: global.names.get(variant.name),
                                    description: variant.description,
                                    parent_name: discriminator_name.clone(),
                                    tag_name,
                                    tag_json_name: tag_json_name.clone(),
                                    tag_json_value: variant.tag_json_value,
                                    fields,
                                },
                            )
                        })?,
                    );
                }

                global.target.write_discriminator(
                    &mut file.target_state,
                    &mut file.buf,
                    Discriminator {
                        name: discriminator_name,
                        description: discriminator.description,
                        tag_name: discriminator.tag_name,
                        tag_json_name: discriminator.tag_json_name,
                        variants,
                    },
                )
            })?
        }
    })
}

fn with_subfile_state<
    'a,
    T: Target,
    F: FnOnce(&mut GlobalState<'a, T>, &mut FileState<T>) -> Result<Expr<T::ExprMeta>>,
>(
    global: &mut GlobalState<'a, T>,
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

    let expr = f(global, subfile_state)?;

    if should_write_out {
        write_out_file(global, &mut subfile_state, &expr)?;
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
