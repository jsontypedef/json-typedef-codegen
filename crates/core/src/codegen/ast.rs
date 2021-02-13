use crate::target::metadata::Metadata;
use crate::target::{NameableKind, OptionalPropertyHandlingStrategy, Target};
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

    Empty {
        metadata: Metadata,
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

    DictOf {
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
    pub has_additional: bool,
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
            Form::Empty => Self::Empty {
                metadata: schema.metadata.clone(),
            },

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

                for (json_name, sub_schema) in &properties.optional {
                    path.push(json_name.into());
                    let ast_name = target.name(NameableKind::Field, path);
                    let ast = Self::new(target, path, sub_schema);
                    path.pop();

                    let ast = match target.strategy().optional_property_handling {
                        // The target natively supports optional properies. The
                        // target will know how to "optional-ify" the underlying
                        // ast directly.
                        OptionalPropertyHandlingStrategy::NativeSupport => ast,

                        // The target only supports optional properties if they
                        // are nullable. We'll wrap the underlying ast with a
                        // NullableOf, assuming it's not already nullable.
                        OptionalPropertyHandlingStrategy::WrapWithNullable => {
                            ast.into_nullable(target, true, sub_schema.metadata.clone())
                        }
                    };

                    fields.push(Field {
                        metadata: sub_schema.metadata.clone(),
                        name: ast_name,
                        json_name: json_name.into(),
                        optional: true,
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

            Form::Values(ref values) => {
                // See comment for Elements for why we singularize the last
                // segment.
                let last = path.pop().expect("empty path");
                path.push(to_singular(&last));

                Ast::DictOf {
                    metadata: schema.metadata.clone(),
                    type_: Box::new(Self::new(target, path, &values.schema)),
                }
                .into_nullable(target, values.nullable, schema.metadata.clone())
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
                            has_additional,
                        } => {
                            variants.push(DiscriminatorVariant {
                                metadata,
                                type_name: name,
                                field_name: variant_field_name,
                                tag_value: tag_value.clone(),
                                has_additional,
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
                .into_nullable(
                    target,
                    discriminator.nullable,
                    schema.metadata.clone(),
                )
            }
        }
    }

    fn into_nullable<T: Target>(self, target: &T, want_nullable: bool, metadata: Metadata) -> Self {
        let strategy = target.strategy();
        let already_nullable = match self {
            Ast::Empty { .. } => true,
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
            Ast::DictOf { .. } => strategy.dicts_are_nullable,
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
