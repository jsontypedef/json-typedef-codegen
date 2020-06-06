use crate::handlebars_helpers;
use crate::statemgr::StateManager;
use failure::Error;
use handlebars::Handlebars;
use inflector::Inflector;
use jtd::{Form, Schema};
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Target {
    pkg_name: String,
    root_name: String,
    out_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct TemplateData {
    package: String,
    imports: HashSet<String>,
    aliases: Vec<TypeAlias>,
    consts: Vec<Const>,
    structs: Vec<Struct>,
    discriminators: Vec<Discriminator>,
}

#[derive(Debug, Serialize)]
struct TypeAlias {
    description: String,
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Const {
    description: String,
    name: String,
    type_: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Struct {
    description: String,
    name: String,
    members: Vec<Member>,
}

#[derive(Debug, Serialize)]
struct Member {
    description: String,
    name: String,
    rename: String,
    required: bool,
    value: String,
}

#[derive(Debug, Serialize)]
struct Discriminator {
    description: String,
    name: String,
    tag: String,
    tag_type: String,
    tag_rename: String,
    members: Vec<DiscriminatorVariant>,
}

#[derive(Debug, Serialize)]
struct DiscriminatorVariant {
    name: String,
    tag_value: String,
}

impl super::Target for Target {
    fn args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        app.arg(
            clap::Arg::with_name("go-out")
                .long("go-out")
                .help("Go output directory")
                .takes_value(true),
        )
    }

    fn from_args(matches: &clap::ArgMatches) -> Result<Option<Self>, Error> {
        if let Some(out_dir) = matches.value_of("go-out") {
            Ok(Some(Self {
                pkg_name: Path::new(matches.value_of("go-out").unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
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
                package: self.pkg_name.clone(),
                imports: HashSet::new(),
                aliases: vec![],
                consts: vec![],
                structs: vec![],
                discriminators: vec![],
            },
        );

        for (name, sub_schema) in &schema.definitions {
            state.with_path_segment(name.clone(), &|state| {
                state.with_must_emit(true, &|state| Self::emit_ast(state, sub_schema))
            });
        }

        state.with_must_emit(true, &|state| Self::emit_ast(state, schema));

        state.data.aliases.sort_by_key(|a| a.name.clone());
        state.data.consts.sort_by_key(|c| c.name.clone());
        state.data.structs.sort_by_key(|s| s.name.clone());
        state.data.discriminators.sort_by_key(|d| d.name.clone());

        for struct_ in &mut state.data.structs {
            struct_.members.sort_by_key(|v| v.name.clone());
        }

        for discriminator in &mut state.data.discriminators {
            discriminator.members.sort_by_key(|m| m.name.clone());
        }

        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);
        registry.register_helper("comment", Box::new(handlebars_helpers::comment));

        let mut out = File::create(self.out_dir.join("index.go"))?;
        registry.render_template_to_write(
            include_str!("template.go.hbs"),
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
                let name = "interface{}".to_owned();

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
                    format!("*{}", state.definition_name(&definition))
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
                    jtd::form::TypeValue::Boolean => "bool",
                    jtd::form::TypeValue::Float32 => "float32",
                    jtd::form::TypeValue::Float64 => "float64",
                    jtd::form::TypeValue::Int8 => "int8",
                    jtd::form::TypeValue::Uint8 => "uint8",
                    jtd::form::TypeValue::Int16 => "int16",
                    jtd::form::TypeValue::Uint16 => "uint16",
                    jtd::form::TypeValue::Int32 => "int32",
                    jtd::form::TypeValue::Uint32 => "uint32",
                    jtd::form::TypeValue::String => "string",
                    jtd::form::TypeValue::Timestamp => {
                        state.data.imports.insert("time".to_owned());
                        "time.Time"
                    }
                };

                let name = if nullable {
                    format!("*{}", name)
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
                let name = state.name();
                state.data.aliases.push(TypeAlias {
                    description: description(schema),
                    name: name.clone(),
                    value: "string".to_owned(),
                });

                for value in values {
                    state.with_path_segment(name.clone(), &|state| {
                        state.with_path_segment(value.clone(), &|state| {
                            state.data.consts.push(Const {
                                description: enum_description(schema, value),
                                name: state.name(),
                                type_: name.clone(),
                                value: format!("{:?}", value),
                            });

                            "".to_owned()
                        })
                    });
                }

                if nullable {
                    format!("*{}", name)
                } else {
                    name
                }
            }
            Form::Elements(jtd::form::Elements { ref schema, .. }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| Self::emit_ast(state, schema))
                });

                let name = format!("[]{}", sub_name);

                if state.must_emit() {
                    state.data.aliases.push(TypeAlias {
                        description: description(schema),
                        name: state.name(),
                        value: name.clone(),
                    });
                }

                name
            }
            Form::Properties(jtd::form::Properties {
                ref required,
                ref optional,
                ..
            }) => {
                let mut members = vec![];
                for (name, schema) in required {
                    members.push(Member {
                        description: description(schema),
                        name: name.to_pascal_case(),
                        rename: name.clone(),
                        required: true,
                        value: state.with_path_segment(name.clone(), &|state| {
                            Self::emit_ast(state, schema)
                        }),
                    });
                }

                for (name, schema) in optional {
                    members.push(Member {
                        description: description(schema),
                        name: name.to_pascal_case(),
                        rename: name.clone(),
                        required: false,
                        value: state.with_path_segment(name.clone(), &|state| {
                            Self::emit_ast(state, schema)
                        }),
                    });
                }

                state.data.structs.push(Struct {
                    description: description(schema),
                    name: state.name(),
                    members,
                });

                state.name()
            }
            Form::Values(jtd::form::Values { ref schema, .. }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| Self::emit_ast(state, schema))
                });

                let name = format!("map[string]{}", sub_name);

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
                state.data.imports.insert("encoding/json".to_owned());

                let tag_type = state.with_path_segment(discriminator.clone(), &|state| {
                    let tag_type = state.name();

                    for (tag_value, mapping_schema) in mapping {
                        state.with_path_segment(tag_value.clone(), &|state| {
                            state.data.consts.push(Const {
                                description: description(mapping_schema),
                                name: state.name(),
                                type_: tag_type.clone(),
                                value: format!("{:?}", tag_value),
                            });

                            state.name()
                        });
                    }

                    state.data.aliases.push(TypeAlias {
                        description: description(schema),
                        name: state.name(),
                        value: "string".to_owned(),
                    });

                    state.name()
                });

                let mut members = vec![];
                for (tag_value, schema) in mapping {
                    let name = state.with_path_segment(tag_value.clone(), &|state| {
                        Self::emit_ast(state, schema)
                    });

                    members.push(DiscriminatorVariant {
                        name: name,
                        tag_value: format!("{:?}", tag_value),
                    });
                }

                state.data.discriminators.push(Discriminator {
                    description: description(schema),
                    name: state.name(),
                    tag: discriminator.to_pascal_case(),
                    tag_rename: discriminator.clone(),
                    tag_type,
                    members,
                });

                if nullable {
                    format!("*{}", state.name())
                } else {
                    state.name()
                }
            }
        }
    }
}

fn description(schema: &Schema) -> String {
    schema
        .metadata
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned())
        .unwrap_or_default()
}

fn enum_description(schema: &Schema, name: &str) -> String {
    schema
        .metadata
        .get("enumDescriptions")
        .and_then(|v| v.as_object())
        .and_then(|a| a.get(name))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned())
        .unwrap_or_default()
}
