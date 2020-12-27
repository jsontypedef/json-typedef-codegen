mod ast;
mod namespace;

use crate::error::Result;
use crate::target::*;
use jtd::Schema;
use namespace::Namespace;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn codegen<T: Target>(
    target: &T,
    root_name: String,
    schema: &Schema,
    out_dir: &Path,
) -> Result<String> {
    let (root, definitions) = ast::from_schema(target, root_name, schema);

    let mut global_namespace = Namespace::new();
    let root_name = ast_name(&mut global_namespace, &root);
    let definition_names = definitions
        .iter()
        .map(|(name, ast)| (name.clone(), ast_name(&mut global_namespace, ast)))
        .collect();

    let mut global_state = GlobalState {
        file_partitioning: target.file_partitioning(),
        enum_strategy: target.enum_strategy(),
        names: global_namespace,
        target,
        definition_names: &definition_names,
        out_dir,
    };

    let mut file_state = FileState {
        buf: Vec::new(),
        target_state: T::FileState::default(),
    };

    for (name, ast) in definitions {
        _codegen(
            &mut global_state,
            &mut file_state,
            ast,
            definition_names[&name].clone(),
        )?;
    }

    let root_expr = _codegen(&mut global_state, &mut file_state, root, root_name)?;

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
    definition_names: &'a BTreeMap<String, String>,
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
    name: String,
) -> Result<String> {
    Ok(match ast_ {
        ast::Ast::Ref(def) => global.definition_names[&def].clone(),
        ast::Ast::Boolean => global.target.boolean(&mut file.target_state),
        ast::Ast::String => global.target.string(&mut file.target_state),
        ast::Ast::Timestamp => global.target.timestamp(&mut file.target_state),
        ast::Ast::ArrayOf(sub_ast) => {
            let sub_name = ast_name(&mut global.names, &sub_ast);
            let sub_expr = _codegen(global, file, *sub_ast, sub_name)?;
            global.target.array_of(&mut file.target_state, sub_expr)
        }
        ast::Ast::NullableOf(sub_ast) => {
            let sub_name = ast_name(&mut global.names, &sub_ast);
            let sub_expr = _codegen(global, file, *sub_ast, sub_name)?;
            global.target.nullable_of(&mut file.target_state, sub_expr)
        }
        ast::Ast::Alias(alias) => with_subfile_state(global, Some(file), |global, file| {
            let sub_name = ast_name(&mut global.names, &alias.type_);
            let sub_expr = _codegen(global, file, *alias.type_, sub_name)?;

            global.target.write_alias(
                &mut file.target_state,
                &mut file.buf,
                Alias {
                    name,
                    metadata: alias.metadata,
                    type_: sub_expr,
                },
            )
        })?,
        ast::Ast::Enum(enum_) => with_subfile_state(global, Some(file), |global, file| {
            let mut variant_names = Namespace::new();
            let mut variants = vec![];

            for variant in enum_.variants {
                let variant_name = match global.enum_strategy {
                    EnumStrategy::Modularized => variant_names.get(variant.name),
                    EnumStrategy::Unmodularized => global.names.get(variant.name),
                };

                variants.push(EnumVariant {
                    name: variant_name,
                    metadata: variant.metadata,
                    json_value: variant.json_value,
                })
            }

            global.target.write_enum(
                &mut file.target_state,
                &mut file.buf,
                Enum {
                    name,
                    metadata: enum_.metadata,
                    variants: variants,
                },
            )
        })?,
        ast::Ast::Struct(struct_) => with_subfile_state(global, Some(file), |global, file| {
            let mut field_names = Namespace::new();
            let mut fields = Vec::new();

            for field in struct_.fields {
                let field_name = field_names.get(field.name);
                let sub_name = ast_name(&mut global.names, &field.type_);

                fields.push(StructField {
                    name: field_name,
                    json_name: field.json_name,
                    metadata: field.metadata,
                    optional: field.optional,
                    type_: _codegen(global, file, field.type_, sub_name)?,
                });
            }

            global.target.write_struct(
                &mut file.target_state,
                &mut file.buf,
                Struct {
                    name,
                    metadata: struct_.metadata,
                    has_additional: struct_.has_additional,
                    fields,
                },
            )
        })?,
        ast::Ast::Discriminator(discriminator) => {
            with_subfile_state(global, Some(file), |global, file| {
                // Clone these as variables to avoid issues with partial moves
                // of discriminator.
                let tag_name = discriminator.tag_name.clone();
                let tag_json_name = discriminator.tag_json_name.clone();

                // First, set aside names for the variants, so we can write out
                // the discriminator first.
                //
                // Writing the discriminator before the variants can help deal
                // with dynamic languages (e.g. Python) where you can't extend a
                // class unless it's already been defined.
                let mut variant_names = BTreeMap::new();
                for (tag_value, variant) in &discriminator.variants {
                    variant_names.insert(tag_value.clone(), global.names.get(variant.name.clone()));
                }

                let discriminator_out = global.target.write_discriminator(
                    &mut file.target_state,
                    &mut file.buf,
                    Discriminator {
                        name: name.clone(),
                        metadata: discriminator.metadata,
                        tag_name: discriminator.tag_name,
                        tag_json_name: discriminator.tag_json_name,
                        variants: variant_names.clone(),
                    },
                )?;

                for (tag_value, variant) in discriminator.variants {
                    with_subfile_state(global, Some(file), |global, file| {
                        // todo: dedupe this logic with struct stuff above?
                        let mut field_names = Namespace::new();
                        let mut fields = Vec::new();

                        // Set aside a name for the discriminator tag,
                        // because it will probably be in the same namespace
                        // as the fields of the variant.
                        let tag_name = field_names.get(tag_name.clone());

                        for field in variant.fields {
                            let field_name = field_names.get(field.name);
                            let sub_name = ast_name(&mut global.names, &field.type_);
                            let sub_ast = _codegen(global, file, field.type_, sub_name)?;

                            fields.push(StructField {
                                name: field_name,
                                json_name: field.json_name,
                                metadata: field.metadata,
                                optional: field.optional,
                                type_: sub_ast,
                            });
                        }

                        global.target.write_discriminator_variant(
                            &mut file.target_state,
                            &mut file.buf,
                            DiscriminatorVariant {
                                name: variant_names[&tag_value].clone(),
                                metadata: variant.metadata,
                                parent_name: name.clone(),
                                tag_name,
                                tag_json_name: tag_json_name.clone(),
                                tag_json_value: variant.tag_json_value,
                                fields,
                            },
                        )
                    })?;
                }

                Ok(discriminator_out)
            })?
        }
    })
}

fn ast_name(namespace: &mut Namespace, ast: &ast::Ast) -> String {
    match ast {
        ast::Ast::Alias(ast::Alias { name, .. }) => namespace.get(name.clone()),
        ast::Ast::Enum(ast::Enum { name, .. }) => namespace.get(name.clone()),
        ast::Ast::Struct(ast::Struct { name, .. }) => namespace.get(name.clone()),
        ast::Ast::Discriminator(ast::Discriminator { name, .. }) => namespace.get(name.clone()),

        _ => "".into(), // not an ast node that has a name
    }
}

fn with_subfile_state<
    'a,
    T: Target,
    F: FnOnce(&mut GlobalState<'a, T>, &mut FileState<T>) -> Result<String>,
>(
    global: &mut GlobalState<'a, T>,
    file_state: Option<&mut FileState<T>>,
    f: F,
) -> Result<String> {
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
    type_: &str,
) -> Result<()> {
    let out_file_name = match global.file_partitioning {
        FilePartitioning::FilePerType(ref extension) => Path::new(type_).with_extension(extension),
        FilePartitioning::SingleFile(ref file_name) => Path::new(file_name).to_path_buf(),
    };

    let mut out_file = File::create(global.out_dir.join(out_file_name))?;

    global
        .target
        .write_preamble(&mut file.target_state, &mut out_file)?;
    out_file.write_all(&file.buf)?;

    Ok(())
}
