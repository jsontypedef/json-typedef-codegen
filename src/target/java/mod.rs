mod ast;
mod render;

use crate::root_name::root_name_from_input_name;
use anyhow::Result;
use clap::{App, Arg};
use jtd::Schema;

pub fn with_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("java-out")
            .long("java-out")
            .help("Java output directory")
            .takes_value(true)
            .requires("java-pkg")
            .requires("java-json-lib"),
    )
    .arg(
        Arg::with_name("java-pkg")
            .long("java-pkg")
            .help("Java output package")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("java-json-lib")
            .long("java-json-lib")
            .help("Java JSON library to use")
            .takes_value(true)
            .possible_values(&["jackson"]),
    )
}

pub fn codegen(input: &str, arg_matches: &clap::ArgMatches, schema: &Schema) -> Result<()> {
    // If the user did not supply the "out" argument, do not generate code.
    if !arg_matches.is_present("java-out") {
        return Ok(());
    }

    let root_name = root_name_from_input_name(input);
    let out_dir = arg_matches.value_of("java-out").unwrap();

    let ast = ast::Ast::new(
        root_name,
        arg_matches.value_of("java-pkg").unwrap().to_owned(),
        schema,
    );
    render::render(out_dir, ast)?;

    Ok(())
}
