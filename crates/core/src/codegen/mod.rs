mod ast;
mod namespace;

use crate::error::Result;
use crate::target::{
    DiscriminatorVariantInfo, EnumMember, EnumMemberNamingStrategy, Expr, Field,
    FilePartitioningStrategy, Item, Strategy, Target,
};
use ast::{Ast, SchemaAst};
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
    let schema_ast = SchemaAst::new(target, root_name, schema);
    let mut code_generator = CodeGenerator::new(target, out_dir);

    code_generator.codegen(schema_ast)

    // let (root, definitions) = ast::from_schema(target, root_name, schema);

    // let mut global_namespace = Namespace::new();
    // let root_name = ast_name(&mut global_namespace, &root);
    // let definition_names = definitions
    //     .iter()
    //     .map(|(name, ast)| (name.clone(), ast_name(&mut global_namespace, ast)))
    //     .collect();

    // let mut global_state = GlobalState {
    //     file_partitioning: target.file_partitioning(),
    //     enum_strategy: target.enum_strategy(),
    //     names: global_namespace,
    //     target,
    //     definition_names: &definition_names,
    //     out_dir,
    // };

    // let mut file_state = FileState {
    //     buf: Vec::new(),
    //     target_state: T::FileState::default(),
    // };

    // for (name, ast) in definitions {
    //     _codegen(
    //         &mut global_state,
    //         &mut file_state,
    //         ast,
    //         definition_names[&name].clone(),
    //     )?;
    // }

    // let root_expr = _codegen(&mut global_state, &mut file_state, root, root_name)?;

    // // If we are doing single-file file partitioning, then no schema will ever
    // // write itself out to a file. We will need to flush the single file out
    // // here, now that all code has been generated.
    // if let FilePartitioning::SingleFile(_) = global_state.file_partitioning {
    //     write_out_file(&global_state, &mut file_state, &root_expr)?;
    // }

    // Ok(root_expr)
}

struct CodeGenerator<'a, T> {
    target: &'a T,
    out_dir: &'a Path,
    strategy: Strategy,
    definition_names: BTreeMap<String, String>,
}

struct FileData<T> {
    buf: Vec<u8>,
    state: T,
}

impl<'a, T: Target> CodeGenerator<'a, T> {
    pub fn new(target: &'a T, out_dir: &'a Path) -> Self {
        Self {
            target,
            out_dir,
            strategy: target.strategy(),
            definition_names: BTreeMap::new(),
        }
    }

    pub fn codegen(&mut self, schema_ast: SchemaAst) -> Result<String> {
        let mut global_namespace = Namespace::new();

        // Before generating any code, set aside names for the top-level nodes.
        // Because namespaces are first-come, first-served, setting these names
        // aside ensures they have the best chance of having a non-mangled name.
        //
        // To that end, we also do the root name before any of the definitions.
        // We give the highest priority to the root name.
        let root_name = self.ast_name(&mut global_namespace, &schema_ast.root);
        for (name, ast) in &schema_ast.definitions {
            let ast_name = self.ast_name(&mut global_namespace, ast);
            self.definition_names.insert(name.clone(), ast_name);
        }

        // If the target is using FilePerType partitioning, then this state
        // won't actually be used at all. If it's using SingleFile partitioning,
        // then this is the only file state that will be used.
        let mut root_file_data = FileData {
            buf: Vec::new(),
            state: T::FileState::default(),
        };

        self.codegen_ast(
            &mut global_namespace,
            &mut root_file_data,
            root_name.clone(),
            schema_ast.root,
        )?;
        for (name, ast) in schema_ast.definitions {
            let ast_name = self.definition_names[&name].clone();
            self.codegen_ast(&mut global_namespace, &mut root_file_data, ast_name, ast)?;
        }

        // If we are doing SingleFile partitioning, then no schema will ever
        // write itself out to a file. We will need to flush the single file out
        // here, now that all code has been generated.
        if let FilePartitioningStrategy::SingleFile(_) = self.strategy.file_partitioning {
            self.write_file(&mut root_file_data, &root_name)?;
        }

        Ok(root_name)
    }

    fn codegen_ast(
        &self,
        global_namespace: &mut Namespace,
        file_data: &mut FileData<T::FileState>,
        ast_name: String,
        ast: Ast,
    ) -> Result<String> {
        Ok(match ast {
            // Ref nodes are a special sort of "expr-like" node, where we
            // already know what the name of the expression is; it's the name of
            // the definition.
            Ast::Ref { definition, .. } => self.definition_names[&definition].clone(),

            // The remaining "expr-like" node types just build up strings and
            // possibly alter the per-file state (usually in order to add
            // "imports" to the file).
            Ast::Boolean { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Boolean)
            }
            Ast::Int8 { metadata } => self.target.expr(&mut file_data.state, metadata, Expr::Int8),
            Ast::Uint8 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Uint8)
            }
            Ast::Int16 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Int16)
            }
            Ast::Uint16 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Uint16)
            }
            Ast::Int32 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Int32)
            }
            Ast::Uint32 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Uint32)
            }
            Ast::Float32 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Float32)
            }
            Ast::Float64 { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Float64)
            }
            Ast::String { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::String)
            }
            Ast::Timestamp { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Timestamp)
            }
            Ast::ArrayOf { metadata, type_ } => {
                let sub_name = self.ast_name(global_namespace, &type_);
                let sub_expr = self.codegen_ast(global_namespace, file_data, sub_name, *type_)?;

                self.target
                    .expr(&mut file_data.state, metadata, Expr::ArrayOf(sub_expr))
            }
            Ast::NullableOf { metadata, type_ } => {
                let sub_name = self.ast_name(global_namespace, &type_);
                let sub_expr = self.codegen_ast(global_namespace, file_data, sub_name, *type_)?;

                self.target
                    .expr(&mut file_data.state, metadata, Expr::NullableOf(sub_expr))
            }

            // Now the "item-like" node types. For these, the target is given a
            // named item to generate. The target can either generate code for
            // this item (indicated by returning None), or it may decide to not
            // generate any code at all, and instead return a prefab name for
            // the generated type (indicated by returning Some(s), where s is
            // the name of the prefab type).
            //
            // This functionality exists so that targets can support having
            // "overridden" type names (usually via some metadata-based
            // target-specific extension).
            //
            // We need to distinguish between these two cases because when a
            // file-per-type target returns a prefab name, it's important we not
            // try to create a new file for that prefab name; the user will
            // handle making sure that file and type exists.
            //
            // Most of the logic for handling this is done through with_subfile.
            Ast::Alias {
                metadata, type_, ..
            } => self.with_subfile(ast_name.clone(), file_data, |file_data| {
                let sub_name = self.ast_name(global_namespace, &type_);
                let sub_type = self.codegen_ast(global_namespace, file_data, sub_name, *type_)?;

                self.target.item(
                    &mut file_data.buf,
                    &mut file_data.state,
                    Item::Alias {
                        metadata,
                        name: ast_name,
                        type_: sub_type,
                    },
                )
            })?,

            Ast::Enum {
                metadata, members, ..
            } => self.with_subfile(ast_name.clone(), file_data, |file_data| {
                // A namespace for member names. Unused if the target has
                // unmodularized enum member names.
                let mut member_names = Namespace::new();
                let mut enum_members = Vec::new(); // members to pass to target
                for member in members {
                    enum_members.push(EnumMember {
                        name: match self.strategy.enum_member_naming {
                            EnumMemberNamingStrategy::Modularized => &mut member_names,
                            EnumMemberNamingStrategy::Unmodularized => global_namespace,
                        }
                        .get(member.name),
                        json_value: member.json_value,
                    });
                }

                self.target.item(
                    &mut file_data.buf,
                    &mut file_data.state,
                    Item::Enum {
                        metadata,
                        name: ast_name,
                        members: enum_members,
                    },
                )
            })?,

            Ast::Struct {
                metadata,
                has_additional,
                fields,
                ..
            } => {
                self.with_subfile(ast_name.clone(), file_data, |file_data| {
                    let mut field_names = Namespace::new();
                    let mut struct_fields = Vec::new(); // fields to pass to target
                    for field in fields {
                        let field_name = field_names.get(field.name);

                        let sub_name = self.ast_name(global_namespace, &field.type_);
                        let sub_ast =
                            self.codegen_ast(global_namespace, file_data, sub_name, field.type_)?;

                        struct_fields.push(Field {
                            metadata: field.metadata,
                            name: field_name,
                            json_name: field.json_name,
                            optional: field.optional,
                            type_: sub_ast,
                        });
                    }

                    self.target.item(
                        &mut file_data.buf,
                        &mut file_data.state,
                        Item::Struct {
                            metadata,
                            name: ast_name,
                            has_additional,
                            fields: struct_fields,
                        },
                    )
                })?
            }

            Ast::Discriminator {
                metadata,
                tag_field_name,
                tag_json_name,
                variants,
                ..
            } => self.with_subfile(ast_name.clone(), file_data, |file_data| {
                // Reassigning this, because `ast_name` is a bit ambiguous when
                // we're generating both a discriminator and a set of
                // discriminator variants.
                let discriminator_name = ast_name.clone();

                // A namespace for the discriminator tag, as well as each
                // variant, within the context of the discriminator itself.
                let mut discriminator_field_names = Namespace::new();
                let discriminator_tag_field_name =
                    discriminator_field_names.get(tag_field_name.clone());

                // Set aside names for each the variants, so we can generate the
                // discriminator first.
                //
                // We need to generate the discriminator first because some
                // languages (e.g. Python) want to implement variants as
                // subclasses of the discriminator superclass, but you can't
                // define subclass before the superclass.
                let mut variant_names = Vec::new();
                let mut variant_infos = Vec::new();
                for variant in &variants {
                    let type_name = global_namespace.get(variant.type_name.clone());

                    variant_names.push(type_name.clone());
                    variant_infos.push(DiscriminatorVariantInfo {
                        type_name,
                        field_name: discriminator_field_names.get(variant.field_name.clone()),
                        tag_value: variant.tag_value.clone(),
                    });
                }

                let returned_discriminator_name = self.target.item(
                    &mut file_data.buf,
                    &mut file_data.state,
                    Item::Discriminator {
                        metadata,
                        name: discriminator_name.clone(),
                        tag_field_name: discriminator_tag_field_name,
                        tag_json_name: tag_json_name.clone(),
                        variants: variant_infos,
                    },
                )?;

                // The name that the variants will know the discriminator type
                // by. We want to support having the top-level discriminator be
                // able to customize its name and have the variants be able to
                // use that name.
                let discriminator_name_for_variants = returned_discriminator_name
                    .clone()
                    .unwrap_or(discriminator_name);

                // Now generate each of the variants.
                for (i, variant) in variants.into_iter().enumerate() {
                    self.with_subfile(variant_names[i].clone(), file_data, |file_data| {
                        // A for the discriminator tag and the fields of the
                        // variant.
                        let mut variant_field_names = Namespace::new();

                        // This value will always be the same as
                        // `discriminator_tag_field_name`, but we make sure to `get`
                        // it from the namespace here so we don't give the same name
                        // to any of the other fields in the variant.
                        let variant_tag_field_name =
                            variant_field_names.get(tag_field_name.clone());

                        let mut variant_fields = Vec::new();
                        for field in variant.fields {
                            let field_name = variant_field_names.get(field.name);

                            let sub_name = self.ast_name(global_namespace, &field.type_);
                            let sub_ast = self.codegen_ast(
                                global_namespace,
                                file_data,
                                sub_name,
                                field.type_,
                            )?;

                            variant_fields.push(Field {
                                metadata: field.metadata,
                                name: field_name,
                                json_name: field.json_name,
                                optional: field.optional,
                                type_: sub_ast,
                            });
                        }

                        self.target.item(
                            &mut file_data.buf,
                            &mut file_data.state,
                            Item::DiscriminatorVariant {
                                metadata: variant.metadata,
                                name: variant_names[i].clone(),
                                parent_name: discriminator_name_for_variants.clone(),
                                tag_field_name: variant_tag_field_name,
                                tag_json_name: tag_json_name.clone(),
                                tag_value: variant.tag_value,
                                fields: variant_fields,
                            },
                        )
                    })?;
                }

                Ok(returned_discriminator_name)
            })?,
        })

        //     //     ast::Ast::Discriminator(discriminator) => {
        //     //         with_subfile_state(global, Some(file), |global, file| {
        //     //             // Clone these as variables to avoid issues with partial moves
        //     //             // of discriminator.
        //     //             let tag_name = discriminator.tag_name.clone();
        //     //             let tag_json_name = discriminator.tag_json_name.clone();

        //     //             // First, set aside names for the variants, so we can write out
        //     //             // the discriminator first.
        //     //             //
        //     //             // Writing the discriminator before the variants can help deal
        //     //             // with dynamic languages (e.g. Python) where you can't extend a
        //     //             // class unless it's already been defined.
        //     //             let mut variant_names = BTreeMap::new();
        //     //             for (tag_value, variant) in &discriminator.variants {
        //     //                 variant_names.insert(tag_value.clone(), global.names.get(variant.name.clone()));
        //     //             }

        //     //             let discriminator_out = global.target.write_discriminator(
        //     //                 &mut file.target_state,
        //     //                 &mut file.buf,
        //     //                 Discriminator {
        //     //                     name: name.clone(),
        //     //                     metadata: discriminator.metadata,
        //     //                     tag_name: discriminator.tag_name,
        //     //                     tag_json_name: discriminator.tag_json_name,
        //     //                     variants: variant_names.clone(),
        //     //                 },
        //     //             )?;

        //     //             for (tag_value, variant) in discriminator.variants {
        //     //                 with_subfile_state(global, Some(file), |global, file| {
        //     //                     // todo: dedupe this logic with struct stuff above?
        //     //                     let mut field_names = Namespace::new();
        //     //                     let mut fields = Vec::new();

        //     //                     // Set aside a name for the discriminator tag,
        //     //                     // because it will probably be in the same namespace
        //     //                     // as the fields of the variant.
        //     //                     let tag_name = field_names.get(tag_name.clone());

        //     //                     for field in variant.fields {
        //     //                         let field_name = field_names.get(field.name);
        //     //                         let sub_name = ast_name(&mut global.names, &field.type_);
        //     //                         let sub_ast = _codegen(global, file, field.type_, sub_name)?;

        //     //                         fields.push(StructField {
        //     //                             name: field_name,
        //     //                             json_name: field.json_name,
        //     //                             metadata: field.metadata,
        //     //                             optional: field.optional,
        //     //                             type_: sub_ast,
        //     //                         });
        //     //                     }

        //     //                     global.target.write_discriminator_variant(
        //     //                         &mut file.target_state,
        //     //                         &mut file.buf,
        //     //                         DiscriminatorVariant {
        //     //                             name: variant_names[&tag_value].clone(),
        //     //                             metadata: variant.metadata,
        //     //                             parent_name: name.clone(),
        //     //                             tag_name,
        //     //                             tag_json_name: tag_json_name.clone(),
        //     //                             tag_json_value: variant.tag_json_value,
        //     //                             fields,
        //     //                         },
        //     //                     )
        //     //                 })?;
        //     //             }

        //     //             Ok(discriminator_out)
    }

    fn ast_name(&self, namespace: &mut Namespace, ast: &Ast) -> String {
        match ast {
            Ast::Alias { name, .. } => namespace.get(name.clone()),
            Ast::Enum { name, .. } => namespace.get(name.clone()),
            Ast::Struct { name, .. } => namespace.get(name.clone()),
            Ast::Discriminator { name, .. } => namespace.get(name.clone()),
            _ => "".into(), // not a node that has a name
        }
    }

    fn with_subfile<F>(
        &self,
        sub_name: String,
        file_data: &mut FileData<T::FileState>,
        f: F,
    ) -> Result<String>
    where
        F: FnOnce(&mut FileData<T::FileState>) -> Result<Option<String>>,
    {
        let mut default_file_data = FileData {
            buf: Vec::new(),
            state: T::FileState::default(),
        };

        let mut sub_file_data = match self.strategy.file_partitioning {
            FilePartitioningStrategy::FilePerType(_) => &mut default_file_data,
            FilePartitioningStrategy::SingleFile(_) => file_data,
        };

        let returned_name = f(&mut sub_file_data)?;

        match (&self.strategy.file_partitioning, returned_name) {
            // If we're generating a file per type, and the target did not
            // return a prefab name, then we need to generate a new file with
            // the contents of what the target generated.
            (&FilePartitioningStrategy::FilePerType(_), None) => {
                self.write_file(&mut sub_file_data, &sub_name)?;
                Ok(sub_name)
            }

            // If instead we're in single-file mode (but again with no prefab
            // name), then we don't need to write out a file.
            (&FilePartitioningStrategy::SingleFile(_), None) => Ok(sub_name),

            // If a prefab name was returned, then in no circumstance do we
            // write out a file, and we will have the rest of codegen use the
            // returned name.
            (_, Some(prefab_name)) => Ok(prefab_name),
        }
    }

    // fn with_subfile_state<
    //     'a,
    //     T: Target,
    //     F: FnOnce(&mut GlobalState<'a, T>, &mut FileState<T>) -> Result<Option<String>>,
    // >(
    //     global: &mut GlobalState<'a, T>,
    //     file_state: Option<&mut FileState<T>>,
    //     f: F,
    // ) -> Result<Option<String>> {
    //     let mut default_file_state = FileState {
    //         buf: Vec::new(),
    //         target_state: T::FileState::default(),
    //     };

    //     let (mut subfile_state, should_write_out) = match global.strategy.file_partitioning {
    //         FilePartitioningStrategy::FilePerType(_) => (&mut default_file_state, true),
    //         FilePartitioningStrategy::SingleFile(_) => {
    //             let should_write_out = file_state.is_none();
    //             (
    //                 file_state.unwrap_or(&mut default_file_state),
    //                 should_write_out,
    //             )
    //         }
    //     };

    //     let expr = f(global, subfile_state)?;

    //     // Do not emit a new file if no actual code was generated, as indicated by f
    //     // returning None.
    //     if should_write_out {
    //         if let Some(expr) = expr {
    //             write_out_file(global, &mut subfile_state, &expr)?;
    //         }
    //     }

    //     Ok(expr)
    // }

    fn write_file(&self, file_data: &mut FileData<T::FileState>, type_name: &str) -> Result<()> {
        let file_name = match self.strategy.file_partitioning {
            FilePartitioningStrategy::FilePerType(ref extension) => {
                Path::new(type_name).with_extension(extension)
            }
            FilePartitioningStrategy::SingleFile(ref file_name) => {
                Path::new(file_name).to_path_buf()
            }
        };

        let mut file = File::create(self.out_dir.join(file_name))?;
        self.target
            .item(&mut file, &mut file_data.state, Item::Preamble)?;
        file.write_all(&file_data.buf)?;

        Ok(())
    }
}

// struct GlobalState<'a, T: Target> {
//     strategy: Strategy,
//     namespace: Namespace,
//     target: &'a T,
//     definition_names: &'a BTreeMap<String, String>,
//     out_dir: &'a Path,
// }

// struct FileState<T: Target> {
//     buf: Vec<u8>,
//     target_state: T::FileState,
// }

// fn _codegen<'a, T: Target>(
//     global: &mut GlobalState<'a, T>,
//     file: &mut FileState<T>,
//     ast: Ast,
//     name: String,
// ) -> Result<Option<String>> {
//     Ok(match ast {
//         Ast::Ref { definition, .. } => Some(global.definition_names[&definition].clone()),
//         _ => todo!(),
//     })

//     // Ok(match ast_ {
//     //     ast::Ast::Ref(def) => global.definition_names[&def].clone(),
//     //     ast::Ast::Boolean => global
//     //         .target
//     //         .boolean(&mut file.target_state, schema.metadata),
//     //     ast::Ast::String => global.target.string(&mut file.target_state),
//     //     ast::Ast::Timestamp => global.target.timestamp(&mut file.target_state),
//     //     ast::Ast::ArrayOf(sub_ast) => {
//     //         let sub_name = ast_name(&mut global.names, &sub_ast);
//     //         let sub_expr = _codegen(global, file, *sub_ast, sub_name)?;
//     //         global.target.array_of(&mut file.target_state, sub_expr)
//     //     }
//     //     ast::Ast::NullableOf(nullable_of) => {
//     //         let sub_name = ast_name(&mut global.names, &nullable_of.type_);
//     //         let sub_expr = _codegen(global, file, *nullable_of.type_, sub_name)?;
//     //         global.target.nullable_of(&mut file.target_state, Type {
//     //             metadata: nullable_of.metadata,
//     //             type_: sub_expr,
//     //         })
//     //     }
//     //     ast::Ast::Alias(alias) => with_subfile_state(global, Some(file), |global, file| {
//     //         let sub_name = ast_name(&mut global.names, &alias.type_);
//     //         let sub_expr = _codegen(global, file, *alias.type_, sub_name)?;

//     //         global.target.write_alias(
//     //             &mut file.target_state,
//     //             &mut file.buf,
//     //             Alias {
//     //                 name,
//     //                 metadata: alias.metadata,
//     //                 type_: sub_expr,
//     //             },
//     //         )
//     //     })?,
//     //     ast::Ast::Enum(enum_) => with_subfile_state(global, Some(file), |global, file| {
//     //         let mut variant_names = Namespace::new();
//     //         let mut variants = vec![];

//     //         for variant in enum_.variants {
//     //             let variant_name = match global.enum_strategy {
//     //                 EnumStrategy::Modularized => variant_names.get(variant.name),
//     //                 EnumStrategy::Unmodularized => global.names.get(variant.name),
//     //             };

//     //             variants.push(EnumVariant {
//     //                 name: variant_name,
//     //                 metadata: variant.metadata,
//     //                 json_value: variant.json_value,
//     //             })
//     //         }

//     //         global.target.write_enum(
//     //             &mut file.target_state,
//     //             &mut file.buf,
//     //             Enum {
//     //                 name,
//     //                 metadata: enum_.metadata,
//     //                 variants: variants,
//     //             },
//     //         )
//     //     })?,
//     //     ast::Ast::Struct(struct_) => with_subfile_state(global, Some(file), |global, file| {
//     //         let mut field_names = Namespace::new();
//     //         let mut fields = Vec::new();

//     //         for field in struct_.fields {
//     //             let field_name = field_names.get(field.name);
//     //             let sub_name = ast_name(&mut global.names, &field.type_);

//     //             fields.push(StructField {
//     //                 name: field_name,
//     //                 json_name: field.json_name,
//     //                 metadata: field.metadata,
//     //                 optional: field.optional,
//     //                 type_: _codegen(global, file, field.type_, sub_name)?,
//     //             });
//     //         }

//     //         global.target.write_struct(
//     //             &mut file.target_state,
//     //             &mut file.buf,
//     //             Struct {
//     //                 name,
//     //                 metadata: struct_.metadata,
//     //                 has_additional: struct_.has_additional,
//     //                 fields,
//     //             },
//     //         )
//     //     })?,
//     //     ast::Ast::Discriminator(discriminator) => {
//     //         with_subfile_state(global, Some(file), |global, file| {
//     //             // Clone these as variables to avoid issues with partial moves
//     //             // of discriminator.
//     //             let tag_name = discriminator.tag_name.clone();
//     //             let tag_json_name = discriminator.tag_json_name.clone();

//     //             // First, set aside names for the variants, so we can write out
//     //             // the discriminator first.
//     //             //
//     //             // Writing the discriminator before the variants can help deal
//     //             // with dynamic languages (e.g. Python) where you can't extend a
//     //             // class unless it's already been defined.
//     //             let mut variant_names = BTreeMap::new();
//     //             for (tag_value, variant) in &discriminator.variants {
//     //                 variant_names.insert(tag_value.clone(), global.names.get(variant.name.clone()));
//     //             }

//     //             let discriminator_out = global.target.write_discriminator(
//     //                 &mut file.target_state,
//     //                 &mut file.buf,
//     //                 Discriminator {
//     //                     name: name.clone(),
//     //                     metadata: discriminator.metadata,
//     //                     tag_name: discriminator.tag_name,
//     //                     tag_json_name: discriminator.tag_json_name,
//     //                     variants: variant_names.clone(),
//     //                 },
//     //             )?;

//     //             for (tag_value, variant) in discriminator.variants {
//     //                 with_subfile_state(global, Some(file), |global, file| {
//     //                     // todo: dedupe this logic with struct stuff above?
//     //                     let mut field_names = Namespace::new();
//     //                     let mut fields = Vec::new();

//     //                     // Set aside a name for the discriminator tag,
//     //                     // because it will probably be in the same namespace
//     //                     // as the fields of the variant.
//     //                     let tag_name = field_names.get(tag_name.clone());

//     //                     for field in variant.fields {
//     //                         let field_name = field_names.get(field.name);
//     //                         let sub_name = ast_name(&mut global.names, &field.type_);
//     //                         let sub_ast = _codegen(global, file, field.type_, sub_name)?;

//     //                         fields.push(StructField {
//     //                             name: field_name,
//     //                             json_name: field.json_name,
//     //                             metadata: field.metadata,
//     //                             optional: field.optional,
//     //                             type_: sub_ast,
//     //                         });
//     //                     }

//     //                     global.target.write_discriminator_variant(
//     //                         &mut file.target_state,
//     //                         &mut file.buf,
//     //                         DiscriminatorVariant {
//     //                             name: variant_names[&tag_value].clone(),
//     //                             metadata: variant.metadata,
//     //                             parent_name: name.clone(),
//     //                             tag_name,
//     //                             tag_json_name: tag_json_name.clone(),
//     //                             tag_json_value: variant.tag_json_value,
//     //                             fields,
//     //                         },
//     //                     )
//     //                 })?;
//     //             }

//     //             Ok(discriminator_out)
//     //         })?
//     //     }
//     // })
// }

// fn ast_name(namespace: &mut Namespace, ast: &Ast) -> String {
//     match ast {
//         Ast::Alias { name, .. } => namespace.get(name.clone()),
//         _ => "".into(), // not a node that has a name
//     }

//     // match ast {
//     //     ast::Ast::Alias(ast::Alias { name, .. }) => namespace.get(name.clone()),
//     //     ast::Ast::Enum(ast::Enum { name, .. }) => namespace.get(name.clone()),
//     //     ast::Ast::Struct(ast::Struct { name, .. }) => namespace.get(name.clone()),
//     //     ast::Ast::Discriminator(ast::Discriminator { name, .. }) => namespace.get(name.clone()),

//     //     _ => "".into(), // not an ast node that has a name
//     // }
// }

// fn write_out_file<'a, T: Target>(
//     global: &GlobalState<'a, T>,
//     file: &mut FileState<T>,
//     type_: &str,
// ) -> Result<()> {
//     let out_file_name = match global.strategy.file_partitioning {
//         FilePartitioningStrategy::FilePerType(ref extension) => Path::new(type_).with_extension(extension),
//         FilePartitioningStrategy::SingleFile(ref file_name) => Path::new(file_name).to_path_buf(),
//     };

//     let mut out_file = File::create(global.out_dir.join(out_file_name))?;
//     global
//         .target
//         .write_item(&mut file.target_state, Item::Preamble, &mut out_file)?;
//     out_file.write_all(&file.buf)?;

//     Ok(())
// }
