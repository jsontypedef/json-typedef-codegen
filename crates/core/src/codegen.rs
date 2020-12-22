use crate::ast;
use crate::namespace::Namespace;
use crate::target::*;
use crate::Result;

use jtd::Schema;

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
        file_partitioning: T::file_partitioning(),
        enum_strategy: T::enum_strategy(),
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
) -> Result<String> {
    Ok(match ast_ {
        ast::Ast::Ref(def) => global.defs[&def].clone(),
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
            let alias_name = global.names.get(alias.name);
            let sub_expr = _codegen(global, file, *alias.type_)?;

            global.target.write_alias(
                &mut file.target_state,
                &mut file.buf,
                Alias {
                    name: alias_name,
                    description: alias.description,
                    type_: sub_expr,
                },
            )
        })?,
        ast::Ast::Enum(enum_) => with_subfile_state(global, Some(file), |global, file| {
            let enum_name = global.names.get(enum_.name);

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
                    name: enum_name,
                    description: enum_.description,
                    variants: variants,
                },
            )
        })?,
        ast::Ast::Struct(struct_) => with_subfile_state(global, Some(file), |global, file| {
            let struct_name = global.names.get(struct_.name);

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
                    name: struct_name,
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
