use crate::statemgr::StateManager;
use failure::Error;
use handlebars::Handlebars;
use inflector::Inflector;
use jtd::{Form, Schema};
use serde::Serialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Target {
    root_name: String,
    out_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct TemplateData {
    aliases: Vec<TypeAlias>,
    structs: Vec<Struct>,
}

#[derive(Debug, Serialize)]
struct TypeAlias {
    description: Vec<String>,
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Struct {
    description: Vec<String>,
    name: String,
    members: Vec<Member>,
}

#[derive(Debug, Serialize)]
struct Member {
    description: Vec<String>,
    name: String,
    required: bool,
    value: String,
}

impl super::Target for Target {
    fn args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        app.arg(
            clap::Arg::with_name("typescript-out")
                .long("typescript-out")
                .help("TypeScript output directory")
                .takes_value(true),
        )
    }

    fn from_args(matches: &clap::ArgMatches) -> Result<Option<Self>, Error> {
        if let Some(out_dir) = matches.value_of("typescript-out") {
            Ok(Some(Self {
                root_name: Path::new(matches.value_of("INPUT").unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".jtd")
                    .to_owned(),
                out_dir: PathBuf::from(out_dir),
            }))
        } else {
            Ok(None)
        }
    }

    fn codegen(&self, schema: &Schema) -> Result<(), Error> {
        let mut state = StateManager::new(
            self.root_name.clone(),
            TemplateData {
                aliases: vec![],
                structs: vec![],
            },
        );

        for (name, sub_schema) in &schema.definitions {
            state.with_path_segment(name.clone(), &|state| {
                state.with_must_emit(true, &|state| Self::emit_ast(state, sub_schema))
            });
        }

        state.with_must_emit(true, &|state| Self::emit_ast(state, schema));

        state.data.aliases.sort_by_key(|a| a.name.clone());

        for struct_ in &mut state.data.structs {
            struct_.members.sort_by_key(|v| v.name.clone());
        }

        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);

        let mut out = File::create(self.out_dir.join("index.ts"))?;
        registry.render_template_to_write(
            include_str!("template.ts.hbs"),
            &Some(state.data),
            &mut out,
        )?;
        Ok(())
    }
}

impl Target {
    fn emit_ast(state: &mut StateManager<TemplateData>, schema: &Schema) -> String {
        match schema.form {
            Form::Empty => {
                let name = "any".to_owned();

                if state.must_emit() {
                    state.data.aliases.push(TypeAlias {
                        description: description(schema),
                        name: state.name(),
                        value: name.clone(),
                    });
                }

                name
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
                            description: vec![],
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
}

fn description(schema: &Schema) -> Vec<String> {
    schema
        .metadata
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| {
            s.to_owned()
                .split("\n")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
