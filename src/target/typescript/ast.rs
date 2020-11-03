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
        include_str!("typescript_reserved_words.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
        "default".to_owned(),
    );
}

pub struct Ast {
    pub version: String,
    pub types: Namespace<Type>,
    pub definition_names: BTreeMap<String, usize>,
}

impl Ast {
    pub fn new(root_name: &str, schema: &Schema) -> Ast {
        let mut state = State::new(
            NamingConvention::new(
                SeparatorStyle::PascalCase,
                include_str!("typescript_reserved_words.txt")
                    .lines()
                    .map(str::to_owned)
                    .collect(),
                "default".to_owned(),
            ),
            Ast {
                version: crate_version!().to_owned(),
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
    TypeAlias(TypeAlias),
    Interface(Interface),
}

pub struct TypeAlias {
    pub description: String,
    pub type_: TypeRef,
}

pub struct Interface {
    pub description: String,
    pub fields: BTreeMap<String, InterfaceField>,
}

pub struct InterfaceField {
    pub description: String,
    pub optional: bool,
    pub type_: TypeRef,
}

pub enum TypeRef {
    Primitive(String),
    Identifier(usize),
    Definition(String),
    UndefinedOr(Box<TypeRef>),
    ArrayOf(Box<TypeRef>),
    ObjectOf(Box<TypeRef>),
    UnionOf(Vec<TypeRef>),
}

fn emit_ast(state: &mut State<Ast>, schema: &Schema) -> TypeRef {
    match schema.form {
        Form::Empty => with_type_wrapper(state, schema, |_state| {
            TypeRef::Primitive("unknown".to_owned())
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
                        form::TypeValue::Float32 => "number",
                        form::TypeValue::Float64 => "number",
                        form::TypeValue::Int8 => "number",
                        form::TypeValue::Uint8 => "number",
                        form::TypeValue::Int16 => "number",
                        form::TypeValue::Uint16 => "number",
                        form::TypeValue::Int32 => "number",
                        form::TypeValue::Uint32 => "number",
                        form::TypeValue::String => "string",
                        form::TypeValue::Timestamp => "string",
                    }
                    .to_owned(),
                )
            })
        }),

        Form::Enum(form::Enum {
            ref values,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |_state| {
                TypeRef::UnionOf(
                    values
                        .iter()
                        .map(|v| TypeRef::Primitive(format!("{:?}", v)))
                        .collect(),
                )
            })
        }),

        Form::Elements(form::Elements {
            schema: ref sub_schema,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                TypeRef::ArrayOf(Box::new(
                    state.with_singularize(|state| emit_ast(state, sub_schema)),
                ))
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
                    Type::Interface(Interface {
                        description: metadata::description(schema),
                        fields: fields,
                    }),
                ))
            })
        }),

        Form::Values(form::Values {
            schema: ref sub_schema,
            nullable,
        }) => with_type_wrapper(state, schema, |state| {
            with_nullable_wrapper(state, nullable, |state| {
                TypeRef::ObjectOf(Box::new(
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
                let mut variants = Vec::new();
                for (name, sub_schema) in mapping {
                    if let Form::Properties(ref properties) = sub_schema.form {
                        let mut fields = properties_fields(state, properties);
                        fields.insert(
                            sanitize_property_name(discriminator.clone()),
                            InterfaceField {
                                description: "".to_owned(),
                                optional: false,
                                type_: TypeRef::Primitive(format!("{:?}", name)),
                            },
                        );

                        variants.push(state.with_path_segment(name.clone(), |state| {
                            let name = state.name();
                            let priority = state.priority();

                            TypeRef::Identifier(state.data_mut().types.insert(
                                priority,
                                name,
                                Type::Interface(Interface {
                                    description: metadata::description(schema),
                                    fields: fields,
                                }),
                            ))
                        }))
                    }
                }

                // We technically could just return a UnionOf the variants. But
                // in almost all cases, the generated code makes more sense if
                // we emit a type alias here.
                let name = state.name();
                let priority = state.priority();

                TypeRef::Identifier(state.data_mut().types.insert(
                    priority,
                    name,
                    Type::TypeAlias(TypeAlias {
                        description: metadata::description(schema),
                        type_: TypeRef::UnionOf(variants),
                    }),
                ))
            })
        }),
    }
}

fn properties_fields(
    state: &mut State<Ast>,
    properties: &form::Properties,
) -> BTreeMap<String, InterfaceField> {
    let mut fields = BTreeMap::new();

    for (name, sub_schema) in &properties.required {
        fields.insert(
            sanitize_property_name(name.clone()),
            state.with_path_segment(name, |state| InterfaceField {
                description: metadata::description(sub_schema),
                optional: false,
                type_: emit_ast(state, sub_schema),
            }),
        );
    }

    for (name, sub_schema) in &properties.optional {
        fields.insert(
            sanitize_property_name(name.clone()),
            state.with_path_segment(name, |state| InterfaceField {
                description: metadata::description(sub_schema),
                optional: true,
                type_: emit_ast(state, sub_schema),
            }),
        );
    }

    fields
}

/// Returns the name of a property as it should appear in a TypeScript
/// interface.
///
/// TypeScript's code generation is unique in that property names must exactly
/// match their JSON names. So we do not have the option to unconditionally
/// mangle names, nor do we have the option to "rename" properties.
///
/// In order to make outputted code as pretty as possible, we will "escape" a
/// property's name (by Debug-formatting it) only if required.
fn sanitize_property_name(s: String) -> String {
    let escaped = format!("{:?}", s);

    if s.is_empty() || !s.chars().nth(0).unwrap().is_ascii_alphabetic() {
        return escaped;
    }

    if !s.chars().all(|c| c == '_' || c.is_ascii_alphanumeric()) {
        return escaped;
    }

    s
}

/// Returns `UndefinedOr[f(state)]` if `nullable`.
fn with_nullable_wrapper<F>(state: &mut State<Ast>, nullable: bool, f: F) -> TypeRef
where
    F: FnOnce(&mut State<Ast>) -> TypeRef,
{
    if !nullable {
        return f(state);
    }

    TypeRef::UndefinedOr(Box::new(f(state)))
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
            Type::TypeAlias(TypeAlias {
                description: metadata::description(schema),
                type_,
            }),
        )),
    }
}
