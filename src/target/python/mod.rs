use crate::metadata;
use crate::state_manager::{root_name_from_input_name, Namespace, State};
use anyhow::Result;
use askama::Template;
use clap::{crate_version, App, Arg};
use inflector::Inflector;
use jtd::{form, Form, Schema};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

lazy_static! {
    static ref RESERVED_WORDS: BTreeSet<String> = vec![
        // Python keywords, drawn from:
        //
        // https://docs.python.org/3/reference/lexical_analysis.html#keywords
        "False", "await", "else", "import", "pass", "None", "break", "except", "in", "raise",
        "True", "class", "finally", "is", "return", "and", "continue", "for", "lambda", "try",
        "as", "def", "from", "nonlocal", "while", "assert", "del", "global", "not", "with",
        "async", "elif", "if", "or", "yield",

        // Classes or identifiers we may import or generate
        "str", "int", "float", "bool", "list", "dict", "dataclasses", "dataclass", "enum",
        "Enum", "typing", "Dict", "List", "Optional", "Union", "_JsonWrapper", "value",
        "value_cls", "cls", "self",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
}

pub fn with_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("python-out")
            .long("python-out")
            .help("Python output directory")
            .takes_value(true),
    )
}

pub fn codegen(input: &str, arg_matches: &clap::ArgMatches, schema: &Schema) -> Result<()> {
    // If the user did not supply the "out" argument, do not generate code.
    if !arg_matches.is_present("python-out") {
        return Ok(());
    }

    let root_name = root_name_from_input_name(input);
    let out_dir = arg_matches.value_of("python-out").unwrap();

    // This is the state we will build up and pass to handlebars.
    let mut state = State::new(TemplateData {
        version: crate_version!().to_owned(),
        imports: BTreeMap::new(),
        classes: BTreeMap::new(),
    });

    // Add imports we always potentially use.
    add_import(&mut state, "typing", "get_origin");
    add_import(&mut state, "typing", "get_args");
    add_import(&mut state, "typing", "Union");

    // Generate root-level data.
    state.with_path_segment(root_name, &|state| {
        state.with_must_emit(&|state| {
            emit_ast(state, schema);
        });
    });

    // Generate definition-level data.
    for (name, sub_schema) in &schema.definitions {
        state.with_path_segment(name, &|state| {
            state.with_must_emit(&|state| {
                emit_ast(state, sub_schema);
            });
        });
    }

    // Output the code.
    fs::write(
        Path::new(out_dir).join("__init__.py"),
        state.data().render()?,
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "template.py", escape = "none")]
struct TemplateData {
    version: String,
    imports: BTreeMap<String, BTreeSet<String>>,
    classes: BTreeMap<String, Class>,
}

enum Class {
    TypeWrapper(TypeWrapper),
    Enum(Enum),
    Dataclass(Dataclass),
    Discriminator(Discriminator),
}

struct TypeWrapper {
    description: String,
    type_: String,
}

struct Enum {
    description: String,
    members: BTreeMap<String, EnumMember>,
}

struct EnumMember {
    description: String,
    value: String,
}

struct Dataclass {
    description: String,
    fields: BTreeMap<String, DataclassField>,
}

struct DataclassField {
    description: String,
    json_name: String,
    type_: String,
}

struct Discriminator {
    description: String,
    discriminator_name: String,
    discriminator_json_name: String,
    variants: BTreeMap<String, DiscriminatorVariant>,
}

struct DiscriminatorVariant {
    discriminator_value: String,
    type_: String,
}

fn emit_ast<'a>(state: &mut State<'a, TemplateData>, schema: &'a Schema) -> String {
    match schema.form {
        Form::Empty => {
            add_import(state, "typing", "Any");
            type_or_wrapper(state, schema, "Any".to_owned())
        }

        Form::Ref(form::Ref {
            ref definition,
            nullable,
        }) => {
            let type_ = definition.to_pascal_case();
            let type_ = if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", type_)
            } else {
                type_
            };

            type_or_wrapper(state, schema, type_)
        }

        Form::Type(form::Type {
            ref type_value,
            nullable,
        }) => {
            let type_ = match type_value {
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
            };

            let type_ = if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", type_)
            } else {
                type_.to_owned()
            };

            type_or_wrapper(state, schema, type_)
        }

        Form::Enum(form::Enum {
            ref values,
            nullable,
        }) => {
            let mut members = BTreeMap::new();
            for value in values {
                members.insert_name(
                    &RESERVED_WORDS,
                    value.to_screaming_snake_case(),
                    EnumMember {
                        description: "".to_owned(),
                        value: format!("{:?}", value),
                    },
                );
            }

            add_import(state, "enum", "Enum");

            let name = state.name();
            let name = state.data_mut().classes.insert_name(
                &RESERVED_WORDS,
                name,
                Class::Enum(Enum {
                    description: "".to_owned(),
                    members,
                }),
            );

            let type_ = if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", name)
            } else {
                name
            };

            type_or_wrapper(state, schema, type_)
        }

        Form::Elements(form::Elements {
            schema: ref sub_schema,
            nullable,
        }) => {
            add_import(state, "typing", "List");
            let type_ = format!(
                "List[{}]",
                state.with_singularize(&|state| emit_ast(state, sub_schema))
            );

            let type_ = if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", type_)
            } else {
                type_
            };

            type_or_wrapper(state, schema, type_)
        }

        Form::Properties(form::Properties {
            ref required,
            ref optional,
            nullable,
            ..
        }) => {
            let mut fields = BTreeMap::new();

            for (name, sub_schema) in required {
                fields.insert_name(
                    &RESERVED_WORDS,
                    name.to_snake_case(),
                    state.with_path_segment(name, &|state| DataclassField {
                        description: "".to_owned(),
                        json_name: format!("{:?}", name),
                        type_: emit_ast(state, sub_schema),
                    }),
                );
            }

            for (name, sub_schema) in optional {
                fields.insert_name(
                    &RESERVED_WORDS,
                    name.to_snake_case(),
                    state.with_path_segment(name, &|state| {
                        add_import(state, "typing", "Optional");

                        DataclassField {
                            description: metadata::description(sub_schema),
                            json_name: format!("{:?}", name),
                            type_: format!("Optional[{}]", emit_ast(state, sub_schema)),
                        }
                    }),
                );
            }

            add_import(state, "dataclasses", "dataclass");

            let name = state.name();
            let name = state.data_mut().classes.insert_name(
                &RESERVED_WORDS,
                name,
                Class::Dataclass(Dataclass {
                    description: metadata::description(schema),
                    fields,
                }),
            );

            if nullable {
                add_import(state, "typing", "Optional");
                type_or_wrapper(state, schema, format!("Optional[{}]", name))
            } else {
                name
            }
        }

        Form::Values(form::Values {
            schema: ref sub_schema,
            nullable,
        }) => {
            add_import(state, "typing", "Dict");
            let type_ = format!("Dict[str, {}]", emit_ast(state, sub_schema));

            let type_ = if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", type_)
            } else {
                type_
            };

            type_or_wrapper(state, schema, type_)
        }

        Form::Discriminator(form::Discriminator {
            ref discriminator,
            ref mapping,
            nullable,
            ..
        }) => {
            let mut variants = BTreeMap::new();
            for (name, sub_schema) in mapping {
                variants.insert_name(
                    &RESERVED_WORDS,
                    name.to_snake_case(),
                    DiscriminatorVariant {
                        discriminator_value: format!("{:?}", name),
                        type_: state.with_path_segment(name, &|state| emit_ast(state, sub_schema)),
                    }
                );
            }

            add_import(state, "dataclasses", "dataclass");
            add_import(state, "typing", "Optional");

            let name = state.name();
            let name = state.data_mut().classes.insert_name(
                &RESERVED_WORDS,
                name,
                Class::Discriminator(Discriminator {
                    description: metadata::description(schema),
                    discriminator_name: discriminator.to_snake_case(),
                    discriminator_json_name: format!("{:?}", discriminator),
                    variants,
                }),
            );

            if nullable {
                type_or_wrapper(state, schema, format!("Optional[{}]", name))
            } else {
                name
            }
        }
    }
}

fn type_or_wrapper<'a>(
    state: &mut State<'a, TemplateData>,
    schema: &Schema,
    type_: String,
) -> String {
    if state.must_emit() {
        let name = state.name();

        add_import(state, "dataclasses", "dataclass");

        state.data_mut().classes.insert_name(
            &RESERVED_WORDS,
            name,
            Class::TypeWrapper(TypeWrapper {
                description: metadata::description(schema),
                type_,
            }),
        )
    } else {
        type_
    }
}

fn add_import<'a>(state: &mut State<'a, TemplateData>, module: &str, identifier: &str) {
    state
        .data_mut()
        .imports
        .entry(module.to_owned())
        .or_default()
        .insert(identifier.to_owned());
}
