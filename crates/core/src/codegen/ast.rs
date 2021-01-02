use crate::target::metadata::Metadata;
use crate::target::{NameableKind, Target};
use jtd::form::TypeValue;
use jtd::{Form, Schema};
use std::collections::BTreeMap;
use teeter_inflector::string::singularize::to_singular;

#[derive(Debug)]
pub struct SchemaAst {
    pub root: Ast,
    pub definitions: BTreeMap<String, Ast>,
}

impl SchemaAst {
    pub fn new<T: Target>(target: &T, root_name: String, schema: &Schema) -> Self {
        let root = Ast::new_top_level(target, root_name, schema);
        let definitions = schema
            .definitions
            .iter()
            .map(|(name, sub_schema)| {
                (
                    name.clone(),
                    Ast::new_top_level(target, name.clone(), sub_schema),
                )
            })
            .collect();

        Self { root, definitions }
    }
}

#[derive(Debug)]
pub enum Ast {
    Ref {
        metadata: Metadata,
        definition: String,
    },

    Boolean {
        metadata: Metadata,
    },

    Int8 {
        metadata: Metadata,
    },

    Uint8 {
        metadata: Metadata,
    },

    Int16 {
        metadata: Metadata,
    },

    Uint16 {
        metadata: Metadata,
    },

    Int32 {
        metadata: Metadata,
    },

    Uint32 {
        metadata: Metadata,
    },

    Float32 {
        metadata: Metadata,
    },

    Float64 {
        metadata: Metadata,
    },

    String {
        metadata: Metadata,
    },

    Timestamp {
        metadata: Metadata,
    },

    ArrayOf {
        metadata: Metadata,
        type_: Box<Ast>,
    },

    NullableOf {
        metadata: Metadata,
        type_: Box<Ast>,
    },

    Alias {
        metadata: Metadata,
        name: String,
        type_: Box<Ast>,
    },

    Enum {
        metadata: Metadata,
        name: String,
        members: Vec<EnumMember>,
    },

    Struct {
        metadata: Metadata,
        name: String,
        has_additional: bool,
        fields: Vec<Field>,
    },

    Discriminator {
        metadata: Metadata,
        name: String,
        tag_field_name: String,
        tag_json_name: String,
        variants: Vec<DiscriminatorVariant>,
    },
}

#[derive(Debug)]
pub struct EnumMember {
    pub name: String,
    pub json_value: String,
}

#[derive(Debug)]
pub struct DiscriminatorVariant {
    pub metadata: Metadata,
    pub type_name: String,
    pub field_name: String,
    pub tag_value: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub metadata: Metadata,
    pub name: String,
    pub json_name: String,
    pub optional: bool,
    pub type_: Ast,
}

impl Ast {
    fn new_top_level<T: Target>(target: &T, name: String, schema: &Schema) -> Self {
        let ast = Self::new(target, &mut vec![name.clone()], schema);

        match ast {
            Self::Alias { .. }
            | Self::Enum { .. }
            | Self::Struct { .. }
            | Self::Discriminator { .. } => ast,
            _ => Ast::Alias {
                metadata: schema.metadata.clone(),
                name: target.name(NameableKind::Type, &[name]),
                type_: Box::new(ast),
            },
        }
    }

    fn new<T: Target>(target: &T, path: &mut Vec<String>, schema: &Schema) -> Self {
        match schema.form {
            Form::Ref(ref ref_) => Self::Ref {
                metadata: schema.metadata.clone(),
                definition: ref_.definition.clone(),
            }
            .into_nullable(target, ref_.nullable, schema.metadata.clone()),

            Form::Type(ref type_) => match type_.type_value {
                TypeValue::Boolean => Self::Boolean {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Int8 => Self::Int8 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Uint8 => Self::Uint8 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Int16 => Self::Int16 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Uint16 => Self::Uint16 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Int32 => Self::Int32 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Uint32 => Self::Uint32 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Float32 => Self::Float32 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Float64 => Self::Float64 {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::String => Self::String {
                    metadata: schema.metadata.clone(),
                },
                TypeValue::Timestamp => Self::Timestamp {
                    metadata: schema.metadata.clone(),
                },
            }
            .into_nullable(target, type_.nullable, schema.metadata.clone()),

            Form::Enum(ref enum_) => {
                let mut members = Vec::new();
                for value in &enum_.values {
                    path.push(value.into());
                    members.push(EnumMember {
                        name: target.name(NameableKind::EnumMember, path),
                        json_value: value.into(),
                    });
                    path.pop();
                }

                Ast::Enum {
                    metadata: schema.metadata.clone(),
                    name: target.name(NameableKind::Type, path),
                    members,
                }
                .into_nullable(target, enum_.nullable, schema.metadata.clone())
            }

            Form::Elements(ref elements) => {
                // It's common for schemas that have an "elements" property to
                // have a plural name for that property, but we don't want to
                // generate []Users and UsersThing, we want to generate []User
                // and UserThing.
                //
                // So we singularize the last name of the path, so that a path
                // like:
                //
                // foo -> bar -> users -> thing -> ...
                //
                // Becomes:
                //
                // foo -> bar -> user -> thing -> ...
                //
                // We here count on the invariant that the inputted path is
                // never empty.
                let last = path.pop().expect("empty path");
                path.push(to_singular(&last));

                Ast::ArrayOf {
                    metadata: schema.metadata.clone(),
                    type_: Box::new(Self::new(target, path, &elements.schema)),
                }
                .into_nullable(target, elements.nullable, schema.metadata.clone())
            }

            Form::Properties(ref properties) => {
                let mut fields = Vec::new();
                for (json_name, sub_schema) in &properties.required {
                    path.push(json_name.into());
                    let ast_name = target.name(NameableKind::Field, path);
                    let ast = Self::new(target, path, sub_schema);
                    path.pop();

                    fields.push(Field {
                        metadata: sub_schema.metadata.clone(),
                        name: ast_name,
                        json_name: json_name.into(),
                        optional: false,
                        type_: ast,
                    });
                }

                Ast::Struct {
                    metadata: schema.metadata.clone(),
                    name: target.name(NameableKind::Type, path),
                    has_additional: properties.additional,
                    fields,
                }
                .into_nullable(
                    target,
                    properties.nullable,
                    schema.metadata.clone(),
                )
            }

            Form::Discriminator(ref discriminator) => {
                let discriminator_name = target.name(NameableKind::Type, path);

                path.push(discriminator.discriminator.clone());
                let tag_field_name = target.name(NameableKind::Field, path);
                path.pop();

                let mut variants = Vec::new();
                for (tag_value, sub_schema) in &discriminator.mapping {
                    path.push(tag_value.into());
                    let variant_field_name = target.name(NameableKind::Field, path);
                    let variant_ast = Self::new(target, path, sub_schema);
                    path.pop();

                    // The remainder of this code relies on the fact that a
                    // mapping can only be a non-nullable schema of the
                    // properties form, which means we know it can only be an
                    // Ast::Struct.
                    //
                    // This invariant on JTD mappings is specified here:
                    //
                    // https://tools.ietf.org/html/rfc8927#section-2.2.8
                    match variant_ast {
                        Ast::Struct {
                            metadata,
                            name,
                            fields,
                            ..
                        } => {
                            variants.push(DiscriminatorVariant {
                                metadata,
                                type_name: name,
                                field_name: variant_field_name,
                                tag_value: tag_value.clone(),
                                fields,
                            });
                        }
                        _ => unreachable!(),
                    }
                }

                Ast::Discriminator {
                    metadata: schema.metadata.clone(),
                    name: discriminator_name,
                    tag_field_name,
                    tag_json_name: discriminator.discriminator.clone(),
                    variants,
                }
            }

            _ => todo!(),
        }
    }

    fn into_nullable<T: Target>(self, target: &T, want_nullable: bool, metadata: Metadata) -> Self {
        let strategy = target.strategy();
        let already_nullable = match self {
            Ast::Ref { .. } => false, // just to be safe, assume references are always non-null
            Ast::Boolean { .. } => strategy.booleans_are_nullable,
            Ast::Int8 { .. } => strategy.int8s_are_nullable,
            Ast::Uint8 { .. } => strategy.uint8s_are_nullable,
            Ast::Int16 { .. } => strategy.int16s_are_nullable,
            Ast::Uint16 { .. } => strategy.uint16s_are_nullable,
            Ast::Int32 { .. } => strategy.int32s_are_nullable,
            Ast::Uint32 { .. } => strategy.uint32s_are_nullable,
            Ast::Float32 { .. } => strategy.float32s_are_nullable,
            Ast::Float64 { .. } => strategy.float64s_are_nullable,
            Ast::String { .. } => strategy.strings_are_nullable,
            Ast::Timestamp { .. } => strategy.timestamps_are_nullable,
            Ast::Enum { .. } => strategy.enums_are_nullable,
            Ast::ArrayOf { .. } => strategy.arrays_are_nullable,
            Ast::NullableOf { .. } => true,
            Ast::Alias { .. } => strategy.aliases_are_nullable,
            Ast::Struct { .. } => strategy.structs_are_nullable,
            Ast::Discriminator { .. } => strategy.discriminators_are_nullable,
        };

        if !want_nullable || already_nullable {
            return self;
        }

        Self::NullableOf {
            metadata,
            type_: Box::new(self),
        }
    }
}

// pub fn from_schema<T: Target>(
//     target: &T,
//     root_name: String,
//     schema: &Schema,
// ) -> (Ast, BTreeMap<String, Ast>) {
//     let root = from_schema_top_level(target, root_name, schema);
//     let definitions = schema
//         .definitions
//         .iter()
//         .map(|(name, sub_schema)| {
//             (
//                 name.clone(),
//                 from_schema_top_level(target, name.clone(), sub_schema),
//             )
//         })
//         .collect();

//     (root, definitions)
// }

// fn from_schema_top_level<T: Target>(target: &T, name: String, schema: &Schema) -> Ast {
//     let ast = _from_schema(target, &mut vec![name.clone()], schema);
//     match ast {
//         Ast::Alias(_) | Ast::Enum(_) | Ast::Struct(_) | Ast::Discriminator(_) => ast,
//         _ => Ast::Alias(Alias {
//             name: T::name_type(&[name]),
//             metadata: schema.metadata.clone(),
//             type_: Box::new(ast),
//         }),
//     }
// }

// fn _from_schema<T: Target>(target: &T, path: &mut Vec<String>, schema: &Schema) -> Ast {
//     match schema.form {
//         Form::Ref(form::Ref {
//             ref definition,
//             nullable,
//         }) => with_nullable::<T>(
//             nullable,
//             Ast::Ref(Ref {
//                 metadata: schema.metadata,
//                 definition: definition.clone(),
//             }),
//         ),
//         Form::Type(form::Type {
//             ref type_value,
//             nullable,
//         }) => with_nullable::<T>(
//             nullable,
//             match type_value {
//                 TypeValue::Boolean => Ast::Boolean(Boolean {
//                     metadata: schema.metadata,
//                 }),
//                 TypeValue::String => Ast::String(StringNode {
//                     metadata: schema.metadata,
//                 }),
//                 TypeValue::Timestamp => Ast::Timestamp(Timestamp {
//                     metadata: schema.metadata,
//                 }),
//                 _ => todo!(),
//             },
//         ),
//         Form::Enum(form::Enum {
//             ref values,
//             nullable,
//         }) => {
//             let mut variants = vec![];
//             for value in values {
//                 path.push(value.into());
//                 variants.push(EnumVariant {
//                     name: T::name_enum_variant(path),
//                     metadata: schema.metadata.clone(),
//                     json_value: value.into(),
//                 });
//                 path.pop();
//             }

//             let name = T::name_type(path);
//             with_nullable::<T>(
//                 nullable,
//                 Ast::Enum(Enum {
//                     name,
//                     metadata: schema.metadata.clone(),
//                     variants,
//                 }),
//             )
//         }
//         Form::Elements(form::Elements {
//             schema: ref sub_schema,
//             nullable,
//         }) => {
//             // singularize the last path segment, because it turns out it was
//             // referring to a list of things, and thus we presume the name may
//             // be in the plural.
//             let last = path.pop().expect("todo: top-level elements");
//             path.push(to_singular(&last));

//             with_nullable::<T>(
//                 nullable,
//                 Ast::ArrayOf(ArrayOf {
//                     metadata: schema.metadata,
//                     type_: Box::new(_from_schema(target, path, sub_schema)),
//                 }),
//             )
//         }
//         Form::Properties(form::Properties {
//             ref required,
//             optional: _,
//             additional,
//             nullable,
//             ..
//         }) => {
//             let struct_name = T::name_type(path);

//             let mut fields = Vec::new();
//             for (name, sub_schema) in required {
//                 // Determine the field's name.
//                 path.push(name.clone());
//                 let field_name = T::name_field(path);
//                 path.pop();

//                 // Generate an expr representing sub_schema
//                 path.push(name.clone());
//                 let field_ast = _from_schema(target, path, sub_schema);
//                 path.pop();

//                 // Add the expr to the set of fields for the struct
//                 fields.push(StructField {
//                     name: field_name,
//                     json_name: name.clone(),
//                     metadata: sub_schema.metadata.clone(),
//                     optional: false,
//                     type_: field_ast,
//                 });
//             }

//             with_nullable::<T>(
//                 nullable,
//                 Ast::Struct(Struct {
//                     name: struct_name,
//                     metadata: schema.metadata.clone(),
//                     has_additional: additional,
//                     fields,
//                 }),
//             )
//         }
//         Form::Discriminator(form::Discriminator {
//             ref discriminator,
//             ref mapping,
//             nullable,
//         }) => {
//             let discriminator_name = T::name_type(path);

//             path.push(discriminator.clone());
//             let tag_name = T::name_field(path);
//             path.pop();

//             let mut variants = BTreeMap::new();
//             for (tag_value, sub_schema) in mapping {
//                 path.push(tag_value.clone());
//                 let sub_expr = _from_schema(target, path, sub_schema);
//                 path.pop();

//                 // We know we are returning a struct here, because
//                 // sub_schema must be of the properties form.
//                 let struct_ = match sub_expr {
//                     Ast::Struct(s) => s,
//                     _ => unreachable!(),
//                 };

//                 variants.insert(
//                     tag_value.clone(),
//                     DiscriminatorVariant {
//                         name: struct_.name,
//                         metadata: sub_schema.metadata.clone(),
//                         tag_name: tag_name.clone(),
//                         tag_json_name: discriminator.clone(),
//                         tag_json_value: tag_value.clone(),
//                         fields: struct_.fields,
//                     },
//                 );
//             }

//             with_nullable::<T>(
//                 nullable,
//                 Ast::Discriminator(Discriminator {
//                     name: discriminator_name,
//                     tag_name: tag_name.clone(),
//                     tag_json_name: discriminator.clone(),
//                     metadata: schema.metadata.clone(),
//                     variants,
//                 }),
//             )
//         }
//         _ => todo!(),
//     }
// }

// fn with_nullable<T: Target>(nullable: bool, ast: Ast) -> Ast {
//     let ast_must_be_nullable = match ast {
//         Ast::Boolean => T::booleans_are_nullable(),
//         Ast::String => T::strings_are_nullable(),
//         Ast::Timestamp => T::timestamps_are_nullable(),
//         Ast::ArrayOf(_) => T::arrays_are_nullable(),
//         Ast::Alias(_) => T::aliases_are_nullable(),
//         Ast::Enum(_) => T::enums_are_nullable(),
//         Ast::Struct(_) => T::structs_are_nullable(),
//         Ast::Discriminator(_) => T::discriminators_are_nullable(),
//         Ast::Ref(_) => false, // this could be refined, but would require doing an extra pass
//         Ast::NullableOf(_) => true,
//     };

//     // If the user didn't ask for nullable, or if ast is already nullable, just
//     // return ast.
//     if !nullable || ast_must_be_nullable {
//         ast
//     } else {
//         Ast::NullableOf {
//             type_: Box::new(ast),
//         }
//     }
// }
