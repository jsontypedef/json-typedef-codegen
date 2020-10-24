mod ast;
// mod ir;
mod render;

use crate::root_name::root_name_from_input_name;
use anyhow::Result;
use clap::{App, Arg};
use jtd::Schema;
use std::fs::File;
use std::path::Path;

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
    let mut out = File::create(Path::join(Path::new(out_dir), "__init__.py"))?;

    let ast = ast::Ast::new(root_name, schema);
    render::render(&mut out, ast)?;

    Ok(())
}
