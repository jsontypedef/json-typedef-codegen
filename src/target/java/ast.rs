use crate::metadata;
use crate::namespace::Namespace;
use crate::naming_convention::{NamingConvention, SeparatorStyle};
use crate::state::State;
use clap::crate_version;
use jtd::{form, Form, Schema};
use lazy_static::lazy_static;
use std::collections::BTreeMap;

lazy_static! {
    static ref ENUM_NAMING_CONVENTION: NamingConvention = NamingConvention::new(
        SeparatorStyle::ScreamingSnakeCase,
        include_str!("java_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default_name".to_owned(),
    );
    static ref FIELD_NAMING_CONVENTION: NamingConvention = NamingConvention::new(
        SeparatorStyle::CamelCase,
        include_str!("java_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default_name".to_owned(),
    );
    static ref VARIANT_NAMING_CONVENTION: NamingConvention = NamingConvention::new(
        SeparatorStyle::PascalCase,
        include_str!("java_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default_name".to_owned(),
    );
}

pub struct Ast {
    pub version: String,
    pub package: String,
    pub types: Namespace<Type>,
    pub definition_names: BTreeMap<String, usize>,
}

impl Ast {
    pub fn new(root_name: &str, package_name: String, schema: &Schema) -> Ast {
        let mut state = State::new(
            NamingConvention::new(
                SeparatorStyle::PascalCase,
                include_str!("java_reserved_words.txt")
                    .lines()
                    .map(str::to_owned)
                    .collect(),
                "default".to_owned(),
            ),
            Ast {
                version: crate_version!().to_owned(),
                package: package_name,
                types: Namespace::new(),
                definition_names: BTreeMap::new(),
            },
        );

        // Generate root-level data.
        state.with_path_segment(root_name, |state| {
            state.with_priority(0, |state| {
                state.with_must_emit(|state| {
                    emit_ast(state, schema);
                });
            });
        });

        // Generate definition-level data.
        for (name, sub_schema) in &schema.definitions {
            state.with_path_segment(name, |state| {
                state.with_priority(1, |state| {
                    state.with_must_emit(|state| {
                        match emit_ast(state, sub_schema) {
                            TypeRef::Identifier(id) => {
                                state
                                    .data_mut()
                                    .definition_names
                                    .insert(name.to_owned(), id);
                            }
                            _ => unreachable!(),
                        };
                    });
                });
            });
        }

        state.into_data()
    }
}

pub enum Type {
    TypeWrapper(TypeWrapper),
    Enum(Enum),
    Bean(Bean),
    Discriminator(Discriminator),
}

pub struct TypeWrapper {
    pub description: String,
    pub type_: TypeRef,
}

pub struct Enum {
    pub description: String,
    pub members: BTreeMap<String, EnumMember>,
}

pub struct EnumMember {
    pub description: String,
    pub value: String,
}

pub struct Bean {
    pub description: String,
    pub ignore_unknown: bool,
    pub fields: BTreeMap<String, BeanField>,
}

pub struct Discriminator {
    pub description: String,
    pub discriminator_json_name: String,
    pub variants: BTreeMap<String, DiscriminatorVariant>,
}

pub struct DiscriminatorVariant {
    pub description: String,
    pub discriminator_value: String,
    pub fields: BTreeMap<String, BeanField>,
}

pub enum BeanField {
    Declaration(BeanFieldDeclaration),
    Getter(BeanFieldGetter),
    Setter(BeanFieldSetter),
}

pub struct BeanFieldDeclaration {
    pub json_name: String,
    pub omit_if_null: bool,
    pub type_: TypeRef,
}

pub struct BeanFieldGetter {
    pub description: String,
    pub ivar_name: String,
    pub type_: TypeRef,
}

pub struct BeanFieldSetter {
    pub description: String,
    pub ivar_name: String,
    pub type_: TypeRef,
}

#[derive(Clone)]
pub enum TypeRef {
    Primitive(String),
    Identifier(usize),
    Definition(String),
    ListOf(Box<TypeRef>),
    DictOf(Box<TypeRef>),
}

fn emit_ast(state: &mut State<Ast>, schema: &Schema) -> TypeRef {
    match schema.form {
        Form::Empty => with_type_wrapper(state, schema, |_state| {
            TypeRef::Primitive("Object".to_owned())
        }),

        Form::Ref(form::Ref {
            ref definition,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |_state| {
                TypeRef::Definition(definition.to_owned())
            })
        }),

        Form::Type(form::Type {
            ref type_value,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |_state| {
                TypeRef::Primitive(
                    match type_value {
                        form::TypeValue::Boolean => "boolean",
                        form::TypeValue::Float32 => "float",
                        form::TypeValue::Float64 => "double",
                        form::TypeValue::Int8 => "byte",
                        form::TypeValue::Uint8 => "UnsignedByte",
                        form::TypeValue::Int16 => "short",
                        form::TypeValue::Uint16 => "UnsignedShort",
                        form::TypeValue::Int32 => "int",
                        form::TypeValue::Uint32 => "UnsignedInteger",
                        form::TypeValue::String => "String",
                        form::TypeValue::Timestamp => "java.time.OffsetDateTime",
                    }
                    .to_owned(),
                )
            })
        }),

        Form::Enum(form::Enum {
            ref values,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                let mut members = Namespace::new();

                for value in values {
                    members.insert(
                        0,
                        ENUM_NAMING_CONVENTION.get(&[value]),
                        EnumMember {
                            description: metadata::enum_description(schema, value),
                            value: format!("{:?}", value),
                        },
                    );
                }

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().types.insert(
                    priority,
                    name,
                    Type::Enum(Enum {
                        description: metadata::description(schema),
                        members: members.into_lookup_table().collect(),
                    }),
                ))
            })
        }),

        Form::Elements(form::Elements {
            schema: ref sub_schema,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                TypeRef::ListOf(Box::new(nullableify_typeref(
                    state.with_singularize(|state| emit_ast(state, sub_schema)),
                )))
            })
        }),

        Form::Properties(ref properties) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, properties.nullable, |state| {
                let fields = properties_fields(state, &properties);

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().types.insert(
                    priority,
                    name,
                    Type::Bean(Bean {
                        description: metadata::description(schema),
                        ignore_unknown: properties.additional,
                        fields,
                    }),
                ))
            })
        }),

        Form::Values(form::Values {
            schema: ref sub_schema,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                TypeRef::DictOf(Box::new(nullableify_typeref(
                    state.with_singularize(|state| emit_ast(state, sub_schema)),
                )))
            })
        }),

        Form::Discriminator(form::Discriminator {
            ref discriminator,
            ref mapping,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                let mut variants = Namespace::new();
                for (name, sub_schema) in mapping {
                    if let Form::Properties(properties) = &sub_schema.form {
                        variants.insert(
                            0,
                            VARIANT_NAMING_CONVENTION.get(&[discriminator, name]),
                            DiscriminatorVariant {
                                description: metadata::description(sub_schema),
                                discriminator_value: format!("{:?}", name),
                                fields: properties_fields(state, properties),
                            },
                        );
                    }
                }

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().types.insert(
                    priority,
                    name,
                    Type::Discriminator(Discriminator {
                        description: metadata::description(schema),
                        discriminator_json_name: format!("{:?}", discriminator),
                        variants: variants.into_lookup_table().collect(),
                    }),
                ))
            })
        }),
    }
}

fn properties_fields(
    state: &mut State<Ast>,
    properties: &form::Properties,
) -> BTreeMap<String, BeanField> {
    // We need to do this in two passes: first, we determine the names of the
    // instance variables corresponding to each of the properties in the schema.
    //
    // Then, we generate getters and setters for each of these fields.

    // Little utility struct we'll use to hand over info between the first and
    // second pass.
    struct InstanceVar {
        type_: TypeRef,
        json_name: String,
        omit_if_null: bool,
        description: String,
    }

    let mut instance_vars = Namespace::new();

    for (name, sub_schema) in &properties.required {
        let type_ = state.with_path_segment(name, |state| emit_ast(state, sub_schema));

        instance_vars.insert(
            0,
            FIELD_NAMING_CONVENTION.get(&[name]),
            InstanceVar {
                type_,
                json_name: name.clone(),
                omit_if_null: false,
                description: metadata::description(sub_schema),
            },
        );
    }

    for (name, sub_schema) in &properties.optional {
        let type_ =
            nullableify_typeref(state.with_path_segment(name, |state| emit_ast(state, sub_schema)));

        instance_vars.insert(
            0,
            FIELD_NAMING_CONVENTION.get(&[name]),
            InstanceVar {
                type_,
                json_name: name.clone(),
                omit_if_null: true,
                description: metadata::description(sub_schema),
            },
        );
    }

    let mut fields = Namespace::new();
    for (ivar_name, ivar) in instance_vars.into_lookup_table() {
        // Instance variable declaration
        fields.insert(
            0,
            ivar_name.clone(),
            BeanField::Declaration(BeanFieldDeclaration {
                json_name: ivar.json_name.clone(),
                omit_if_null: ivar.omit_if_null,
                type_: ivar.type_.clone(),
            }),
        );

        // Getter
        let getter_name = match ivar.type_ {
            TypeRef::Primitive(ref s) if s == "boolean" => {
                FIELD_NAMING_CONVENTION.get(&["is", &ivar_name])
            }
            _ => FIELD_NAMING_CONVENTION.get(&["get", &ivar_name]),
        };

        fields.insert(
            1,
            getter_name,
            BeanField::Getter(BeanFieldGetter {
                description: format!("Getter for \"{}\".\n\n{}", ivar.json_name, ivar.description),
                ivar_name: ivar_name.clone(),
                type_: ivar.type_.clone(),
            }),
        );

        // Setter
        fields.insert(
            1,
            FIELD_NAMING_CONVENTION.get(&["set", &ivar_name]),
            BeanField::Setter(BeanFieldSetter {
                description: format!("Setter for \"{}\".\n\n{}", ivar.json_name, ivar.description),
                ivar_name: ivar_name.clone(),
                type_: ivar.type_.clone(),
            }),
        );
    }

    fields.into_lookup_table().collect()
}

/// Returns `Optional[f(state)]` if `nullable`.
fn with_nullable_wrapper<F>(state: &mut State<Ast>, nullable: bool, f: F) -> TypeRef
where
    F: FnOnce(&mut State<Ast>) -> TypeRef,
{
    if !nullable {
        return f(state);
    }

    nullableify_typeref(f(state))
}

/// By default, everything in Java is nullable except for the various number
/// primitive types.
///
/// This function is the identity function for everything except for primitive
/// number types, which instead are "autoboxed" by this function.
fn nullableify_typeref(t: TypeRef) -> TypeRef {
    match t {
        TypeRef::Primitive(s) => TypeRef::Primitive(
            match &s[..] {
                "boolean" => "Boolean",
                "byte" => "Byte",
                "short" => "Short",
                "int" => "Integer",
                "float" => "Float",
                "double" => "Double",
                _ => &s,
            }
            .to_owned(),
        ),
        _ => t,
    }
}

/// Returns a type-alised version of `f(state)` if `state.must_emit()` and
/// `f(state)` is not already an identifier.
fn with_type_wrapper<F>(state: &mut State<Ast>, schema: &Schema, f: F) -> TypeRef
where
    F: FnOnce(&mut State<Ast>) -> TypeRef,
{
    if !state.must_emit() {
        return f(state);
    }

    let priority = state.priority();
    let name = state.name();
    let type_ = state.with_path_segment("", f);

    match type_ {
        TypeRef::Identifier(_) => type_,
        _ => TypeRef::Identifier(state.data_mut().types.insert(
            priority,
            name,
            Type::TypeWrapper(TypeWrapper {
                description: metadata::description(schema),
                type_,
            }),
        )),
    }
}
