use failure::Error;
use handlebars::Handlebars;
use jtd::{Form, Schema};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Target {
    out_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct TemplateData {
    aliases: Vec<TypeAlias>,
    enums: Vec<Enum>,
}

#[derive(Debug, Serialize)]
struct TypeAlias {
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Enum {
    name: String,
    variants: Vec<String>,
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
                out_dir: PathBuf::from(out_dir),
            }))
        } else {
            Ok(None)
        }
    }

    fn codegen(&self, schema: &Schema) -> Result<(), Error> {
        let mut data = TemplateData {
            aliases: vec![],
            enums: vec![],
        };

        for (name, sub_schema) in &schema.definitions {
            Self::emit_ast(&mut data, &mut vec![name.to_owned()], sub_schema);
        }

        Self::emit_ast(&mut data, &mut vec!["Root".to_owned()], schema);

        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);

        println!("{:?}", data);
        println!(
            "{}",
            registry.render_template(include_str!("template.rs.hbs"), &Some(data))?
        );
        Ok(())
    }
}

impl Target {
    fn emit_ast(out: &mut TemplateData, path: &mut Vec<String>, schema: &Schema) -> String {
        match schema.form {
            Form::Empty => {
                if path.len() == 1 {
                    out.aliases.push(TypeAlias {
                        name: path[0].to_owned(),
                        value: "serde_json::Value".to_owned(),
                    });
                    path[0].to_owned()
                } else {
                    "serde_json::Value".to_owned()
                }
            }
            Form::Type(jtd::form::Type { ref type_value, .. }) => {
                let name = match type_value {
                    jtd::form::TypeValue::Boolean => "bool",
                    _ => unreachable!(),
                };

                if path.len() == 1 {
                    out.aliases.push(TypeAlias {
                        name: path[0].to_owned(),
                        value: name.to_owned(),
                    });
                    path[0].to_owned()
                } else {
                    name.to_owned()
                }
            }
            Form::Enum(jtd::form::Enum {
                ref values,
                nullable,
            }) => {
                out.enums.push(Enum {
                    name: path[0].to_owned(),
                    variants: values.into_iter().cloned().collect(),
                });

                path[0].to_owned()
            }
            Form::Elements(jtd::form::Elements {
                ref schema,
                nullable,
            }) => {
                let value = format!("Vec<{}>", Self::emit_ast(out, path, schema));

                if path.len() == 1 {
                    out.aliases.push(TypeAlias {
                        name: path[0].to_owned(),
                        value: value,
                    });
                    path[0].to_owned()
                } else {
                    value
                }
            }
            _ => unreachable!(),
        }
    }
}
