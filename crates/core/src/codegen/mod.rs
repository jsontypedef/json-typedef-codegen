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

pub struct CodegenInfo {
    pub root_name: String,
    pub definition_names: BTreeMap<String, String>,
}

pub fn codegen<T: Target>(
    target: &T,
    root_name: String,
    schema: &Schema,
    out_dir: &Path,
) -> Result<CodegenInfo> {
    let schema_ast = SchemaAst::new(target, root_name, schema);
    let mut code_generator = CodeGenerator::new(target, out_dir);

    code_generator.codegen(schema_ast)
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

    pub fn codegen(&mut self, schema_ast: SchemaAst) -> Result<CodegenInfo> {
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

        self.target.item(
            &mut vec![],
            &mut T::FileState::default(),
            Item::Auxiliary {
                out_dir: self.out_dir.to_owned(),
            },
        )?;

        Ok(CodegenInfo {
            root_name,
            definition_names: self.definition_names.clone(),
        })
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
            Ast::Empty { metadata } => {
                self.target
                    .expr(&mut file_data.state, metadata, Expr::Empty)
            }
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
            Ast::DictOf { metadata, type_ } => {
                let sub_name = self.ast_name(global_namespace, &type_);
                let sub_expr = self.codegen_ast(global_namespace, file_data, sub_name, *type_)?;

                self.target
                    .expr(&mut file_data.state, metadata, Expr::DictOf(sub_expr))
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
                                has_additional: variant.has_additional,
                                fields: variant_fields,
                            },
                        )
                    })?;
                }

                Ok(returned_discriminator_name)
            })?,
        })
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
        self.target
            .item(&mut file, &mut file_data.state, Item::Postamble)?;

        Ok(())
    }
}
