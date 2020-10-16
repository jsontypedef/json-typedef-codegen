use crate::handlebars_helpers;
use crate::metadata;
use crate::state_manager::{root_name_from_input_name, Namespace, State};
use anyhow::Result;
use clap::{crate_version, App, Arg};
use handlebars::{no_escape, Handlebars};
use inflector::Inflector;
use jtd::{form, Form, Schema};
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
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

    // Prepare the main output file.
    let mut out = File::create(Path::new(out_dir).join("__init__.py"))?;

    let mut registry = Handlebars::new();
    registry.register_escape_fn(no_escape);
    registry.register_helper("comment", Box::new(handlebars_helpers::comment));

    registry.render_template_to_write(include_str!("template.py.hbs"), state.data(), &mut out)?;

    Ok(())
}

#[derive(Serialize)]
struct TemplateData {
    version: String,
    imports: BTreeMap<String, BTreeSet<String>>,
    classes: BTreeMap<String, Class>,
}

#[derive(Serialize)]
enum Class {
    Dataclass(Dataclass),
    Enum(Enum),
}

#[derive(Serialize)]
struct Dataclass {
    description: String,
    fields: BTreeMap<String, DataclassField>,
    primitive_wrapper_for: Option<String>,
}

#[derive(Serialize)]
struct DataclassField {
    description: String,
    type_: String,
}

#[derive(Serialize)]
struct Enum {
    description: String,
    members: BTreeMap<String, EnumMember>,
}

#[derive(Serialize)]
struct EnumMember {
    description: String,
    value: String,
}

fn emit_ast<'a>(state: &mut State<'a, TemplateData>, schema: &'a Schema) -> String {
    match schema.form {
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
                    name.to_owned(),
                    state.with_path_segment(name, &|state| DataclassField {
                        description: "".to_owned(),
                        type_: emit_ast(state, sub_schema),
                    }),
                );
            }

            for (name, sub_schema) in optional {
                fields.insert_name(
                    &RESERVED_WORDS,
                    name.to_owned(),
                    state.with_path_segment(name, &|state| {
                        add_import(state, "typing", "Optional");

                        DataclassField {
                            description: metadata::description(sub_schema),
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
                    primitive_wrapper_for: None,
                    fields,
                }),
            );

            if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", name)
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
            ref mapping,
            nullable,
            ..
        }) => {
            for (name, sub_schema) in mapping {
                state.with_path_segment(name, &|state| {
                    emit_ast(state, sub_schema);
                });
            }

            let name = state.name();
            let name = state.data_mut().classes.insert_name(
                &RESERVED_WORDS,
                name,
                Class::Dataclass(Dataclass {
                    description: metadata::description(schema),
                    primitive_wrapper_for: None,
                    fields: BTreeMap::new(),
                }),
            );

            if nullable {
                add_import(state, "typing", "Optional");
                format!("Optional[{}]", name)
            } else {
                name
            }
        }

        _ => "TODO".to_owned(),
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
        add_import(state, "typing", "get_origin");
        add_import(state, "typing", "get_args");
        add_import(state, "typing", "Union");
        add_import(state, "typing", "ClassVar");

        state.data_mut().classes.insert_name(
            &RESERVED_WORDS,
            name,
            Class::Dataclass(Dataclass {
                description: metadata::description(schema),
                primitive_wrapper_for: Some(type_),
                fields: BTreeMap::new(),
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
