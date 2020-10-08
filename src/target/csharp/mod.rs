use crate::statemgr::StateManager;
use failure::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use handlebars::Handlebars;
use jtd::{Form, Schema};
use serde::Serialize;
use inflector::Inflector;
use crate::handlebars_helpers;

pub struct Target {
    namespace_name: String,
    root_name: String,
    out_dir: PathBuf,
}

struct TemplateDatas {
    classes: Vec<Class>,
    enums: Vec<Enum>,
}

#[derive(Debug, Serialize)]
struct Class {
    namespace: String,
    name: String,
    extends: Option<String>,
    is_abstract: bool,
    is_primitive_wrapper: bool,
    is_discriminator: bool,
    static_property: Option<StaticProperty>,
    properties: Vec<Property>,
    discriminator: Option<String>,
    discriminator_variants: Vec<DiscriminatorVariant>,
    description: String,
}

#[derive(Debug, Serialize)]
struct DiscriminatorVariant {
    json_name: String,
    class_name: String,
}

#[derive(Debug, Serialize)]
struct StaticProperty {
    name: String,
    value: String,
    rename: String,
}

#[derive(Debug, Serialize)]
struct Property {
    name: String,
    value: String,
    rename: Option<String>,
    description: String,
}

#[derive(Debug, Serialize)]
struct Enum {
    namespace: String,
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
            clap::Arg::with_name("csharp-out")
                .long("csharp-out")
                .help("C# output directory")
                .takes_value(true)
                .requires("csharp-namespace"),
        )
        .arg(
            clap::Arg::with_name("csharp-namespace")
                .long("csharp-namespace")
                .help("The namespace that outputted C# classes and enums should use")
                .takes_value(true),
        )
    }

    fn from_args(matches: &clap::ArgMatches) -> Result<Option<Self>, Error> {
        if let Some(out_dir) = matches.value_of("csharp-out") {
            Ok(Some(Self {
                namespace_name: matches.value_of("csharp-namespace").unwrap().to_owned(),
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
            let mut out = File::create(self.out_dir.join(&class.name).with_extension("cs"))?;
            registry.render_template_to_write(
                include_str!("class.cs.hbs"),
                &Some(class),
                &mut out,
            )?;
        }

        for enum_ in state.data.enums {
            let mut out = File::create(self.out_dir.join(&enum_.name).with_extension("cs"))?;
            registry.render_template_to_write(
                include_str!("enum.cs.hbs"),
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
                let value = "object".to_owned();

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
            Form::Type(jtd::form::Type { ref type_value, nullable, .. }) => {
                let mut value = match type_value {
                    jtd::form::TypeValue::Boolean => "bool",
                    jtd::form::TypeValue::Float32 => "float",
                    jtd::form::TypeValue::Float64 => "double",
                    jtd::form::TypeValue::Int8 => "sbyte",
                    jtd::form::TypeValue::Uint8 => "byte",
                    jtd::form::TypeValue::Int16 => "short",
                    jtd::form::TypeValue::Uint16 => "ushort",
                    jtd::form::TypeValue::Int32 => "int",
                    jtd::form::TypeValue::Uint32 => "uint",
                    jtd::form::TypeValue::String => "string",
                    jtd::form::TypeValue::Timestamp => "DateTime",
                }
                .to_owned();

                // All of the above types are C# "value" types (i.e. structs, as
                // opposed to classes), except for string. So we mark the type
                // as nullable (with the "?" keyword) if the schema is nullable,
                // unless we're emitting a string, which is already nullable.
                if nullable && type_value != &jtd::form::TypeValue::String {
                    value = format!("{}?", value);
                }

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
            Form::Enum(jtd::form::Enum { ref values, .. }) => {
                let variants: Vec<_> = values
                    .into_iter()
                    .map(|v| Variant {
                        name: v.to_pascal_case(),
                        rename: format!("{:?}", v),
                        description: enum_description(schema, v),
                    })
                    .collect();

                state.data.enums.push(Enum {
                    namespace: self.namespace_name.to_owned(),
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

                let value = format!("IList<{}>", sub_name);

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
                        name: name.to_pascal_case(),
                        rename: Some(format!("{:?}", name)),
                        value: state
                            .with_path_segment(name.clone(), &|state| self.emit_ast(state, schema)),
                        description: description(schema),
                    });
                }

                for (name, schema) in optional {
                    properties.push(Property {
                        name: name.to_pascal_case(),
                        rename: Some(format!("{:?}", name)),
                        value: state
                            .with_path_segment(name.clone(), &|state| self.emit_ast(state, schema)),
                        description: description(schema),
                    });
                }

                state.data.classes.push(Class {
                    namespace: self.namespace_name.to_owned(),
                    name: state.name(),
                    extends: None, // see discriminator: this may be overridden later
                    is_abstract: false,
                    is_primitive_wrapper: false,
                    is_discriminator: false,
                    static_property: None, // this may be overridden as well
                    properties,
                    discriminator: None,
                    discriminator_variants: vec![],
                    description: description(schema),
                });

                state.name()
            }
            Form::Values(jtd::form::Values { schema: ref sub_schema, .. }) => {
                let sub_name = state.with_singularize(true, &|state| {
                    state.with_must_emit(false, &|state| self.emit_ast(state, sub_schema))
                });

                let value = format!("IDictionary<string, {}>", sub_name);

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
                        json_name: format!("{:?}", name),
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
                    let mut sub_class = state.data.classes.last_mut().unwrap();
                    sub_class.extends = Some(parent_name.clone());
                    sub_class.static_property = Some(StaticProperty {
                        name: discriminator.to_pascal_case(),
                        value: format!("{:?}", name),
                        rename: format!("{:?}", discriminator),
                    });
                }

                state.data.classes.push(Class {
                    namespace: self.namespace_name.clone(),
                    name: parent_name.clone(),
                    extends: None,
                    is_abstract: true,
                    is_primitive_wrapper: false,
                    is_discriminator: true,
                    static_property: None,
                    properties: vec![],
                    discriminator: Some(format!("{:?}", discriminator)),
                    discriminator_variants: variants,
                    description: description(schema),
                });

                parent_name
            }
        }
    }

    fn primitive_wrapper_class(&self, schema: &Schema, name: String, wrapped_type: String) -> Class {
        Class {
            namespace: self.namespace_name.to_owned(),
            name: name.clone(),
            extends: None,
            is_abstract: false,
            is_primitive_wrapper: true,
            is_discriminator: false,
            static_property: None,
            properties: vec![Property {
                name: "Value".to_owned(),
                value: wrapped_type,
                rename: None,
                description: "".to_owned(),
            }],
            discriminator: None,
            discriminator_variants: vec![],
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
