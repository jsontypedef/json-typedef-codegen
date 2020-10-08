use crate::statemgr::StateManager;
use failure::Error;
use handlebars::Handlebars;
use inflector::Inflector;
use jtd::{Form, Schema};
use serde::Serialize;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::handlebars_helpers;

#[derive(Debug)]
pub struct Target {
    pkg_name: String,
    root_name: String,
    out_dir: PathBuf,
}

#[derive(Debug)]
struct TemplateDatas {
    classes: Vec<Class>,
    enums: Vec<Enum>,
}

#[derive(Debug, Serialize)]
struct Class {
    package: String,
    discriminator: String,
    discriminator_variants: Vec<DiscriminatorVariant>,
    is_abstract: bool,
    name: String,
    extends: String,
    properties: Vec<Property>,
    description: String,
}

#[derive(Debug, Serialize)]
struct DiscriminatorVariant {
    json_name: String,
    class_name: String,
}

#[derive(Debug, Serialize)]
struct Property {
    name: String,
    method_name: String,
    rename: String,
    value: String,
    entire_value: bool,
    description: String,
}

#[derive(Debug, Serialize)]
struct Enum {
    package: String,
    name: String,
    variants: Vec<Variant>,
    description: String,
}

#[derive(Debug, Serialize)]
struct Variant {
    name: String,
    rename: String,
    description: String,
}

impl super::Target for Target {
    fn args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        app.arg(
            clap::Arg::with_name("java-out")
                .long("java-out")
                .help("Java output directory")
                .takes_value(true)
                .requires("java-pkg"),
        )
        .arg(
            clap::Arg::with_name("java-pkg")
                .long("java-pkg")
                .help("The package that outputted Java classes and enums should use")
                .takes_value(true),
        )
    }

    fn from_args(matches: &clap::ArgMatches) -> Result<Option<Self>, Error> {
        if let Some(out_dir) = matches.value_of("java-out") {
            Ok(Some(Self {
                pkg_name: matches.value_of("java-pkg").unwrap().to_owned(),
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
            TemplateDatas {
                classes: vec![],
                enums: vec![],
            },
        );

        for (name, sub_schema) in &schema.definitions {
            state.with_path_segment(name.clone(), &|state| {
                state.with_must_emit(true, &|state| self.emit_ast(state, sub_schema))
            });
        }

        state.with_must_emit(true, &|state| self.emit_ast(state, schema));

        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);
        registry.register_helper("comment", Box::new(handlebars_helpers::comment));

        for class in &mut state.data.classes {
            class.properties.sort_by_key(|p| p.name.clone());
        }

        for enum_ in &mut state.data.enums {
            enum_.variants.sort_by_key(|p| p.name.clone());
        }

        for class in state.data.classes {
            let mut out = File::create(self.out_dir.join(&class.name).with_extension("java"))?;
            registry.render_template_to_write(
                include_str!("class.java.hbs"),
                &Some(class),
                &mut out,
            )?;
        }

        for enum_ in state.data.enums {
            let mut out = File::create(self.out_dir.join(&enum_.name).with_extension("java"))?;
            registry.render_template_to_write(
                include_str!("enum.java.hbs"),
                &Some(enum_),
                &mut out,
            )?;
        }

        Ok(())
    }
}

impl Target {
    fn emit_ast(&self, state: &mut StateManager<TemplateDatas>, schema: &Schema) -> String {
        match schema.form {
            Form::Empty => {
                let value = "Object".to_owned();

                if state.must_emit() {
                    let name = state.name();

                    state
                        .data
                        .classes
                        .push(self.primitive_wrapper_class(schema, name.clone(), value));
                    name
                } else {
                    value
                }
            }
            Form::Ref(jtd::form::Ref { ref definition, .. }) => {
                let value = state.definition_name(&definition);
                if state.must_emit() {
                    let name = state.name();

                    state
                        .data
                        .classes
                        .push(self.primitive_wrapper_class(schema, name.clone(), value));
                    name
                } else {
                    value
                }
            }
            Form::Type(jtd::form::Type { ref type_value, .. }) => {
                let value = match type_value {
                    jtd::form::TypeValue::Boolean => "Boolean",
                    jtd::form::TypeValue::Float32 => "Float",
                    jtd::form::TypeValue::Float64 => "Double",
                    jtd::form::TypeValue::Int8 => "Byte",
                    jtd::form::TypeValue::Uint8 => "Byte",
                    jtd::form::TypeValue::Int16 => "Short",
                    jtd::form::TypeValue::Uint16 => "Short",
                    jtd::form::TypeValue::Int32 => "Integer",
                    jtd::form::TypeValue::Uint32 => "Integer",
                    jtd::form::TypeValue::String => "String",
                    jtd::form::TypeValue::Timestamp => "OffsetDateTime",
                }
                .to_owned();

                if state.must_emit() {
                    let name = state.name();

                    state
                        .data
                        .classes
                        .push(self.primitive_wrapper_class(schema, name.clone(), value));
                    name
                } else {
                    value
                }
            }
            Form::Enum(jtd::form::Enum { ref values, .. }) => {
                let variants: Vec<_> = values
                    .into_iter()
                    .map(|v| Variant {
                        name: v.to_screaming_snake_case(),
                        rename: format!("{:?}", v),
                        description: enum_description(schema, v),
                    })
                    .collect();

                state.data.enums.push(Enum {
                    package: self.pkg_name.clone(),
                    name: state.name(),
                    variants,
                    description: description(schema),
                });

                state.name()
            }
            Form::Elements(jtd::form::Elements { schema: ref sub_schema, .. }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| self.emit_ast(state, sub_schema))
                });

                let value = format!("List<{}>", sub_name);

                if state.must_emit() {
                    let name = state.name();

                    state
                        .data
                        .classes
                        .push(self.primitive_wrapper_class(schema, name.clone(), value));
                    name
                } else {
                    value
                }
            }
            Form::Properties(jtd::form::Properties {
                ref required,
                ref optional,
                ..
            }) => {
                let mut properties = vec![];
                for (name, schema) in required {
                    properties.push(Property {
                        name: name.to_camel_case(),
                        method_name: name.to_pascal_case(),
                        rename: format!("{:?}", name),
                        value: state
                            .with_path_segment(name.clone(), &|state| self.emit_ast(state, schema)),
                        entire_value: false,
                        description: description(schema),
                    });
                }

                for (name, schema) in optional {
                    properties.push(Property {
                        name: name.to_camel_case(),
                        method_name: name.to_pascal_case(),
                        rename: format!("{:?}", name),
                        value: state
                            .with_path_segment(name.clone(), &|state| self.emit_ast(state, schema)),
                        entire_value: false,
                        description: description(schema),
                    });
                }

                state.data.classes.push(Class {
                    package: self.pkg_name.clone(),
                    discriminator: "".to_owned(),
                    discriminator_variants: vec![],
                    is_abstract: false,
                    name: state.name(),
                    extends: "".to_owned(),
                    properties,
                    description: description(schema),
                });

                state.name()
            }
            Form::Values(jtd::form::Values { schema: ref sub_schema, .. }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| self.emit_ast(state, sub_schema))
                });

                let value = format!("Map<String, {}>", sub_name);

                if state.must_emit() {
                    let name = state.name();

                    state
                        .data
                        .classes
                        .push(self.primitive_wrapper_class(schema, name.clone(), value));
                    name
                } else {
                    value
                }
            }
            Form::Discriminator(jtd::form::Discriminator {
                ref discriminator,
                ref mapping,
                ..
            }) => {
                let parent_name = state.name();

                let mut variants = vec![];
                for (name, schema) in mapping {
                    let variant_class_name = state
                        .with_path_segment(name.clone(), &|state| self.emit_ast(state, schema));

                    variants.push(DiscriminatorVariant {
                        json_name: name.clone(),
                        class_name: variant_class_name,
                    });

                    // A bit of a hack, but this step here ensures that the
                    // class generated in the previous emit_ast call immediately
                    // above will extend the class we are about to generate.
                    //
                    // This code relies on the fact that we always emit classes
                    // for "properties" form schemas (mapping values are always
                    // of that form), and that we do "head" / "depth-first" code
                    // generation, ensuring that the last class we generate is
                    // the one corresponding the schema given to emit_ast, and
                    // not some sub-schema.
                    state.data.classes.last_mut().unwrap().extends = parent_name.clone();
                }

                state.data.classes.push(Class {
                    package: self.pkg_name.clone(),
                    discriminator: format!("{:?}", discriminator),
                    discriminator_variants: variants,
                    is_abstract: true,
                    name: parent_name.clone(),
                    extends: "".to_owned(),
                    properties: vec![],
                    description: description(schema),
                });

                parent_name
            }
        }
    }

    fn primitive_wrapper_class(&self, schema: &Schema, name: String, wrapped_type: String) -> Class {
        Class {
            discriminator: "".to_owned(),
            discriminator_variants: vec![],
            package: self.pkg_name.clone(),
            is_abstract: false,
            name,
            extends: "".to_owned(),
            properties: vec![
                Property {
                    name: "value".to_owned(),
                    method_name: "Value".to_owned(),
                    rename: "".to_owned(),
                    value: wrapped_type,
                    entire_value: true,
                    description: "".to_owned(),
                }
            ],
            description: description(schema),
        }
    }
}

fn description(schema: &Schema) -> String {
    schema
        .metadata
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned()
}

fn enum_description(schema: &Schema, name: &str) -> String {
    schema
        .metadata
        .get("enumDescriptions")
        .and_then(|v| v.as_object())
        .and_then(|a| a.get(name))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned()
}
