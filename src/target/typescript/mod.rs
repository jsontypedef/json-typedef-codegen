use crate::metadata;
use crate::state_manager::root_name_from_input_name;
use crate::state_manager::Namespace;
use crate::state_manager::State;
use anyhow::Result;
use askama::Template;
use clap::crate_version;
use inflector::Inflector;
use jtd::{Form, Schema};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

lazy_static! {
    static ref RESERVED_WORDS: BTreeSet<String> = vec![
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "function",
        "if",
        "import",
        "in",
        "instanceof",
        "new",
        "null",
        "return",
        "super",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "typeof",
        "var",
        "void",
        "while",
        "with",
        "as",
        "implements",
        "interface",
        "let",
        "package",
        "private",
        "protected",
        "public",
        "static",
        "yield",
        "any",
        "boolean",
        "constructor",
        "declare",
        "get",
        "module",
        "require",
        "number",
        "set",
        "string",
        "symbol",
        "type",
        "from",
        "of",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
}

pub fn args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    app.arg(
        clap::Arg::with_name("typescript-out")
            .long("typescript-out")
            .help("TypeScript output directory")
            .takes_value(true),
    )
}

pub fn codegen(input: &str, arg_matches: &clap::ArgMatches, schema: &Schema) -> Result<()> {
    // If the user did not supply the "out" argument, do not generate code.
    if !arg_matches.is_present("typescript-out") {
        return Ok(());
    }

    let root_name = root_name_from_input_name(input);
    let out_dir = arg_matches.value_of("python-out").unwrap();

    // This is the state we will build up and pass to handlebars.
    let mut state = State::new(TemplateData {
        version: crate_version!().to_owned(),
        type_declarations: BTreeMap::new(),
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

    // Output the code.
    fs::write(
        Path::new(out_dir).join("__init__.py"),
        state.data().render()?,
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "template.ts", escape = "none")]
struct TemplateData {
    version: String,
    type_declarations: BTreeMap<String, TypeDeclaration>,
}

enum TypeDeclaration {
    TypeAlias(TypeAlias),
    Interface(Interface),
}

struct TypeAlias {
    description: String,
    type_: String,
}

struct Interface {
    description: String,
    members: BTreeMap<String, InterfaceMember>,
}

struct InterfaceMember {
    description: String,
    required: bool,
    type_: String,
}

fn emit_ast<'a>(state: &mut State<'a, TemplateData>, schema: &'a Schema) -> String {
    match schema.form {
        Form::Empty => {
            type_or_wrapper(state, schema, "any".to_owned())
        }
        Form::Ref(jtd::form::Ref {
            ref definition,
            nullable,
        }) => {
            let name = if nullable {
                format!("({} | undefined)", state.definition_name(&definition))
            } else {
                state.definition_name(&definition)
            };

            if state.must_emit() {
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: state.name(),
                    value: name.clone(),
                });
            }

            name
        }
        Form::Type(jtd::form::Type {
            ref type_value,
            nullable,
        }) => {
            let name = match type_value {
                jtd::form::TypeValue::Boolean => "boolean",
                jtd::form::TypeValue::Float32 => "number",
                jtd::form::TypeValue::Float64 => "number",
                jtd::form::TypeValue::Int8 => "number",
                jtd::form::TypeValue::Uint8 => "number",
                jtd::form::TypeValue::Int16 => "number",
                jtd::form::TypeValue::Uint16 => "number",
                jtd::form::TypeValue::Int32 => "number",
                jtd::form::TypeValue::Uint32 => "number",
                jtd::form::TypeValue::String => "string",
                jtd::form::TypeValue::Timestamp => "string",
            };

            let name = if nullable {
                format!("({} | undefined)", name)
            } else {
                name.to_owned()
            };

            if state.must_emit() {
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: state.name(),
                    value: name.clone(),
                })
            }

            name
        }
        Form::Enum(jtd::form::Enum {
            ref values,
            nullable,
        }) => {
            let mut values: Vec<_> = values
                .into_iter()
                .map(|value| format!("{:?}", value))
                .collect();

            values.sort();

            let name = values.join(" | ");
            let name = if nullable {
                format!("({} | undefined)", name)
            } else {
                format!("({})", name)
            };

            if state.must_emit() {
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: state.name(),
                    value: name.clone(),
                });
            }

            name
        }
        Form::Elements(jtd::form::Elements {
            ref schema,
            nullable,
        }) => {
            let sub_name = state.with_singularize(true, &|state| {
                state.with_must_emit(false, &|state| Self::emit_ast(state, schema))
            });

            let name = if nullable {
                format!("({}[] | undefined)", sub_name)
            } else {
                format!("{}[]", sub_name)
            };

            if state.must_emit() {
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: state.name(),
                    value: name.clone(),
                });
            }

            name
        }
        Form::Properties(jtd::form::Properties { nullable, .. }) => {
            let strukt = Self::props_to_struct(state, schema);
            state.data.structs.push(strukt);

            if nullable {
                format!("({} | undefined)", state.name())
            } else {
                state.name()
            }
        }
        Form::Values(jtd::form::Values {
            ref schema,
            nullable,
        }) => {
            let sub_name = state.with_singularize(true, &|state| {
                state.with_must_emit(false, &|state| Self::emit_ast(state, schema))
            });

            let name = if nullable {
                format!("({{[name: string]: {}}} | undefined)", sub_name)
            } else {
                format!("{{[name: string]: {}}}", sub_name)
            };

            if state.must_emit() {
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: state.name(),
                    value: name.clone(),
                });
            }

            name
        }
        Form::Discriminator(jtd::form::Discriminator {
            ref discriminator,
            ref mapping,
            nullable,
        }) => {
            let mut variants = vec![];
            for (name, schema) in mapping {
                variants.push(state.with_path_segment(name.clone(), &|state| {
                    let mut strukt = Self::props_to_struct(state, schema);
                    strukt.members.push(Member {
                        description: "".to_owned(),
                        name: discriminator.clone(),
                        required: true,
                        value: format!("{:?}", name),
                    });

                    state.data.structs.push(strukt);
                    state.name()
                }));
            }

            variants.sort();

            state.data.aliases.push(TypeAlias {
                description: description(schema),
                name: state.name(),
                value: variants.join(" | "),
            });

            if nullable {
                format!("({} | undefined)", state.name())
            } else {
                state.name()
            }
        }
    }
}

fn props_to_struct(state: &mut StateManager<TemplateData>, schema: &Schema) -> Struct {
    if let Form::Properties(jtd::form::Properties {
        ref required,
        ref optional,
        ..
    }) = schema.form
    {
        let mut members = vec![];
        for (name, schema) in required {
            members.push(Member {
                description: description(schema),
                name: name.to_camel_case(),
                required: true,
                value: state
                    .with_path_segment(name.clone(), &|state| Self::emit_ast(state, schema)),
            });
        }

        for (name, schema) in optional {
            members.push(Member {
                description: description(schema),
                name: name.to_camel_case(),
                required: false,
                value: state
                    .with_path_segment(name.clone(), &|state| Self::emit_ast(state, schema)),
            });
        }

        Struct {
            description: description(schema),
            name: state.name(),
            members,
        }
    } else {
        unreachable!("non-properties form schema passed to props_to_struct")
    }
}

fn type_or_wrapper<'a>(
    state: &mut State<'a, TemplateData>,
    schema: &Schema,
    type_: String,
) -> String {
    if state.must_emit() {
        let name = state.name();

        state.data_mut().type_declarations.insert_name(
            &RESERVED_WORDS,
            name,
            TypeDeclaration::TypeAlias(TypeAlias {
                description: metadata::description(schema),
                type_,
            }),
        )
    } else {
        type_
    }
}
