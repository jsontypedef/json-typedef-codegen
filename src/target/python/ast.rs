use crate::metadata;
use crate::namespace::Namespace;
use crate::naming_convention::{NamingConvention, SeparatorStyle};
use crate::state::State;
use clap::crate_version;
use inflector::Inflector;
use jtd::{form, Form, Schema};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};

lazy_static! {
    static ref ENUM_NAMING_CONVENTION: NamingConvention = NamingConvention::new(
        SeparatorStyle::ScreamingSnakeCase,
        include_str!("python_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default".to_owned(),
    );
    static ref FIELD_NAMING_CONVENTION: NamingConvention = NamingConvention::new(
        SeparatorStyle::SnakeCase,
        include_str!("python_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default".to_owned(),
    );
}

pub struct Ast {
    pub version: String,
    pub imports: BTreeMap<String, BTreeSet<String>>,
    pub classes: Namespace<Class>,
    pub definition_names: BTreeMap<String, usize>,
}

impl Ast {
    pub fn new(root_name: &str, schema: &Schema) -> Ast {
        let mut state = State::new(
            NamingConvention::new(
                SeparatorStyle::PascalCase,
                include_str!("python_reserved_words.txt")
                    .lines()
                    .map(str::to_owned)
                    .collect(),
                "default".to_owned(),
            ),
            Ast {
                version: crate_version!().to_owned(),
                imports: BTreeMap::new(),
                classes: Namespace::new(),
                definition_names: BTreeMap::new(),
            },
        );

        // Add imports we always potentially use.
        add_import(&mut state, "typing", "get_origin");
        add_import(&mut state, "typing", "get_args");
        add_import(&mut state, "typing", "Any");
        add_import(&mut state, "typing", "Union");

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

pub enum Class {
    TypeWrapper(TypeWrapper),
    Enum(Enum),
    Dataclass(Dataclass),
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

pub struct Dataclass {
    pub description: String,
    pub fields: BTreeMap<String, DataclassField>,
}

pub struct DataclassField {
    pub description: String,
    pub json_name: String,
    pub type_: TypeRef,
}

pub struct Discriminator {
    pub description: String,
    pub discriminator_name: String,
    pub discriminator_json_name: String,
    pub variants: BTreeMap<String, DiscriminatorVariant>,
}

pub struct DiscriminatorVariant {
    pub discriminator_value: String,
    pub type_: TypeRef,
}

pub enum TypeRef {
    Primitive(String),
    Identifier(usize),
    Definition(String),
    Optional(Box<TypeRef>),
    ListOf(Box<TypeRef>),
    DictOf(Box<TypeRef>),
}

fn emit_ast(state: &mut State<Ast>, schema: &Schema) -> TypeRef {
    match schema.form {
        Form::Empty => with_type_wrapper(state, schema, |state| {
            add_import(state, "typing", "Any");
            TypeRef::Primitive("Any".to_owned())
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
                        form::TypeValue::Boolean => "bool",
                        form::TypeValue::Float32 => "float",
                        form::TypeValue::Float64 => "float",
                        form::TypeValue::Int8 => "int",
                        form::TypeValue::Uint8 => "int",
                        form::TypeValue::Int16 => "int",
                        form::TypeValue::Uint16 => "int",
                        form::TypeValue::Int32 => "int",
                        form::TypeValue::Uint32 => "int",
                        form::TypeValue::String => "str",
                        form::TypeValue::Timestamp => "str",
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

                add_import(state, "enum", "Enum");

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().classes.insert(
                    priority,
                    name,
                    Class::Enum(Enum {
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
                add_import(state, "typing", "List");
                TypeRef::ListOf(Box::new(
                    state.with_singularize(|state| emit_ast(state, sub_schema)),
                ))
            })
        }),

        Form::Properties(form::Properties {
            ref required,
            ref optional,
            nullable,
            ..
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                let mut fields = Namespace::new();

                for (name, sub_schema) in required {
                    fields.insert(
                        0,
                        FIELD_NAMING_CONVENTION.get(&[name]),
                        state.with_path_segment(name, |state| DataclassField {
                            description: metadata::description(sub_schema),
                            json_name: format!("{:?}", name),
                            type_: emit_ast(state, sub_schema),
                        }),
                    );
                }

                for (name, sub_schema) in optional {
                    add_import(state, "typing", "Optional");

                    fields.insert(
                        0,
                        FIELD_NAMING_CONVENTION.get(&[name]),
                        state.with_path_segment(name, |state| DataclassField {
                            description: metadata::description(sub_schema),
                            json_name: format!("{:?}", name),
                            type_: TypeRef::Optional(Box::new(emit_ast(state, sub_schema))),
                        }),
                    );
                }

                add_import(state, "dataclasses", "dataclass");

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().classes.insert(
                    priority,
                    name,
                    Class::Dataclass(Dataclass {
                        description: metadata::description(schema),
                        fields: fields.into_lookup_table().collect(),
                    }),
                ))
            })
        }),

        Form::Values(form::Values {
            schema: ref sub_schema,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                add_import(state, "typing", "Dict");
                TypeRef::DictOf(Box::new(
                    state.with_singularize(|state| emit_ast(state, sub_schema)),
                ))
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
                    variants.insert(
                        0,
                        FIELD_NAMING_CONVENTION.get(&[name]),
                        DiscriminatorVariant {
                            discriminator_value: format!("{:?}", name),
                            type_: state
                                .with_path_segment(name, |state| emit_ast(state, sub_schema)),
                        },
                    );
                }

                add_import(state, "dataclasses", "dataclass");
                add_import(state, "typing", "Optional");

                let name = state.name();
                let priority = state.priority();
                TypeRef::Identifier(state.data_mut().classes.insert(
                    priority,
                    name,
                    Class::Discriminator(Discriminator {
                        description: metadata::description(schema),
                        discriminator_name: discriminator.to_snake_case(),
                        discriminator_json_name: format!("{:?}", discriminator),
                        variants: variants.into_lookup_table().collect(),
                    }),
                ))
            })
        }),
    }
}

/// Returns `Optional[f(state)]` if `nullable`.
fn with_nullable_wrapper<F>(state: &mut State<Ast>, nullable: bool, f: F) -> TypeRef
where
    F: FnOnce(&mut State<Ast>) -> TypeRef,
{
    if !nullable {
        return f(state);
    }

    add_import(state, "typing", "Optional");
    TypeRef::Optional(Box::new(f(state)))
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

    add_import(state, "dataclasses", "dataclass");

    let priority = state.priority();
    let name = state.name();
    let type_ = state.with_path_segment("", f);

    match type_ {
        TypeRef::Identifier(_) => type_,
        _ => TypeRef::Identifier(state.data_mut().classes.insert(
            priority,
            name,
            Class::TypeWrapper(TypeWrapper {
                description: metadata::description(schema),
                type_,
            }),
        )),
    }
}

fn add_import(state: &mut State<Ast>, module: &str, identifier: &str) {
    state
        .data_mut()
        .imports
        .entry(module.to_owned())
        .or_default()
        .insert(identifier.to_owned());
}
