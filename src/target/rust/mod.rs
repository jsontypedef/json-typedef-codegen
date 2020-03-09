use failure::Error;
use handlebars::Handlebars;
use inflector::string::singularize::to_singular;
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
    enums: Vec<Enum>,
    structs: Vec<Struct>,
}

#[derive(Debug, Serialize)]
struct TypeAlias {
    description: Vec<String>,
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Enum {
    description: Vec<String>,
    name: String,
    tag: Option<String>,
    variants: Vec<Variant>,
}

#[derive(Debug, Serialize)]
struct Variant {
    description: Vec<String>,
    name: String,
    rename: String,
    value: Option<String>,
}

#[derive(Debug, Serialize)]
struct Struct {
    description: Vec<String>,
    name: String,
    deny_unknown_fields: bool,
    members: Vec<Member>,
}

#[derive(Debug, Serialize)]
struct Member {
    description: Vec<String>,
    name: String,
    rename: String,
    required: bool,
    value: String,
}

impl super::Target for Target {
    fn args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        app.arg(
            clap::Arg::with_name("rust-out")
                .long("rust-out")
                .help("Rust output directory")
                .takes_value(true),
        )
    }

    fn from_args(matches: &clap::ArgMatches) -> Result<Option<Self>, Error> {
        if let Some(out_dir) = matches.value_of("rust-out") {
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
                enums: vec![],
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
        state.data.enums.sort_by_key(|e| e.name.clone());
        state.data.structs.sort_by_key(|s| s.name.clone());

        for enum_ in &mut state.data.enums {
            enum_.variants.sort_by_key(|v| v.name.clone());
        }

        for struct_ in &mut state.data.structs {
            struct_.members.sort_by_key(|v| v.name.clone());
        }

        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);

        let mut out = File::create(self.out_dir.join("mod.rs"))?;
        registry.render_template_to_write(
            include_str!("template.rs.hbs"),
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
                let name = "serde_json::Value".to_owned();

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
                    format!("Option<{}>", state.definition_name(&definition))
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
                    jtd::form::TypeValue::Float32 => "f32",
                    jtd::form::TypeValue::Float64 => "f64",
                    jtd::form::TypeValue::Int8 => "i8",
                    jtd::form::TypeValue::Uint8 => "u8",
                    jtd::form::TypeValue::Int16 => "i16",
                    jtd::form::TypeValue::Uint16 => "u16",
                    jtd::form::TypeValue::Int32 => "i32",
                    jtd::form::TypeValue::Uint32 => "u32",
                    jtd::form::TypeValue::String => "String",
                    jtd::form::TypeValue::Timestamp => "DateTime<Utc>",
                };

                let name = if nullable {
                    format!("Option<{}>", name)
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
                state.data.enums.push(Enum {
                    description: description(schema),
                    name: state.name(),
                    tag: None,
                    variants: values
                        .into_iter()
                        .map(|value| Variant {
                            description: enum_description(schema, value),
                            name: value.to_pascal_case(),
                            rename: value.clone(),
                            value: None,
                        })
                        .collect(),
                });

                if nullable {
                    format!("Option<{}>", state.name())
                } else {
                    state.name()
                }
            }
            Form::Elements(jtd::form::Elements {
                ref schema,
                nullable,
            }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| Self::emit_ast(state, schema))
                });

                let name = if nullable {
                    format!("Option<Vec<{}>>", sub_name)
                } else {
                    format!("Vec<{}>", sub_name)
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
            Form::Properties(jtd::form::Properties {
                ref required,
                ref optional,
                additional,
                nullable,
                ..
            }) => {
                let mut members = vec![];
                for (name, schema) in required {
                    members.push(Member {
                        description: description(schema),
                        name: name.to_snake_case(),
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
                        name: name.to_snake_case(),
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
                    deny_unknown_fields: !additional,
                    members,
                });

                if nullable {
                    format!("Option<{}>", state.name())
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
                    format!("Option<HashMap<String, {}>>", sub_name)
                } else {
                    format!("HashMap<String, {}>", sub_name)
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
                    variants.push(Variant {
                        description: description(schema),
                        name: name.to_pascal_case(),
                        rename: name.clone(),
                        value: Some(state.with_path_segment(name.clone(), &|state| {
                            Self::emit_ast(state, schema)
                        })),
                    })
                }

                state.data.enums.push(Enum {
                    description: description(schema),
                    name: state.name(),
                    tag: Some(discriminator.clone()),
                    variants,
                });

                if nullable {
                    format!("Option<{}>", state.name())
                } else {
                    state.name()
                }
            }
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

fn enum_description(schema: &Schema, name: &str) -> Vec<String> {
    schema
        .metadata
        .get("enumDescriptions")
        .and_then(|v| v.as_object())
        .and_then(|a| a.get(name))
        .and_then(|v| v.as_str())
        .map(|s| {
            s.to_owned()
                .split("\n")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

struct StateManager<Data> {
    path: Vec<String>,
    singularize: bool,
    must_emit: bool,
    root_name: String,

    pub data: Data,
}

impl<Data> StateManager<Data> {
    pub fn new(root_name: String, initial_data: Data) -> Self {
        StateManager {
            path: vec![],
            singularize: false,
            must_emit: false,
            root_name,
            data: initial_data,
        }
    }

    pub fn must_emit(&self) -> bool {
        self.must_emit
    }

    pub fn definition_name(&self, definition: &str) -> String {
        definition.to_pascal_case()
    }

    pub fn name(&self) -> String {
        if self.path.is_empty() {
            return self.root_name.to_pascal_case();
        }

        let out = self.path.join("_").to_pascal_case();
        if self.singularize {
            to_singular(&out)
        } else {
            out
        }
    }

    pub fn with_path_segment(
        &mut self,
        segment: String,
        f: &dyn Fn(&mut Self) -> String,
    ) -> String {
        self.path.push(segment);
        let out = self.with_must_emit(false, &|state| state.with_singularize(false, f));
        self.path.pop();

        out
    }

    pub fn with_must_emit(&mut self, must_emit: bool, f: &dyn Fn(&mut Self) -> String) -> String {
        let restore = self.must_emit;

        self.must_emit = must_emit;
        let out = f(self);
        self.must_emit = restore;

        out
    }

    pub fn with_singularize(
        &mut self,
        singularize: bool,
        f: &dyn Fn(&mut Self) -> String,
    ) -> String {
        let restore = self.singularize;

        self.singularize = singularize;
        let out = f(self);
        self.singularize = restore;

        out
    }
}
