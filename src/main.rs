use anyhow::{format_err, Result};
use clap::{crate_version, App, AppSettings, Arg};
use jtd::{Schema, SerdeSchema};
use jtd_codegen::target;
use std::convert::TryInto;
use std::fs::File;

fn main() -> Result<()> {
    let app = App::new("jtd-codegen")
        .version(crate_version!())
        .about("Generate code (classes, structs, etc.) from a JTD schema")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("INPUT")
                .help("Input JDDF schema file")
                .last(true)
                .required(true),
        );

    // let app = target::csharp::Target::args(app);
    // let app = target::go::Target::args(app);
    // let app = target::java::Target::args(app);
    // let app = target::rust::Target::args(app);
    // let app = target::typescript::Target::args(app);
    let app = target::python::with_args(app);

    let matches = app.get_matches();

    // Parse out the input schema, and ensure it is valid.
    let input = matches.value_of("INPUT").unwrap();
    let file = File::open(input)?;
    let serde_schema: SerdeSchema = serde_json::from_reader(file)?;
    let schema: Schema = serde_schema
        .try_into()
        .map_err(|err| format_err!("{:?}", err))?;

    target::python::codegen(&input, &matches, &schema)?;

    // let csharp = target::csharp::Target::from_args(&matches)?;
    // let go = target::go::Target::from_args(&matches)?;
    // let java = target::java::Target::from_args(&matches)?;
    // let rust = target::rust::Target::from_args(&matches)?;
    // let typescript = target::typescript::Target::from_args(&matches)?;
    // let python = target::python::Target::from_args(&matches)?;

    // if let Some(csharp) = csharp {
    //     csharp.codegen(&schema)?;
    // }

    // if let Some(go) = go {
    //     go.codegen(&schema)?;
    // }

    // if let Some(java) = java {
    //     java.codegen(&schema)?;
    // }

    // if let Some(rust) = rust {
    //     rust.codegen(&schema)?;
    // }

    // if let Some(typescript) = typescript {
    //     typescript.codegen(&schema)?;
    // }

    // if let Some(python) = python {
    //     python.codegen(&schema)?;
    // }

    Ok(())
}
