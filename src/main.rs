mod target;

use clap::{crate_version, App, AppSettings, Arg};
// use target::rust::Target as Rust;
use failure::{format_err, Error};
use jtd::{Schema, SerdeSchema};
use std::convert::TryInto;
use std::fs::File;
use target::Target;

fn main() -> Result<(), Error> {
    let app = App::new("jtd-codegen")
        .version(crate_version!())
        .about("Generates data structures from JDDF schemas")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("INPUT")
                .help("Input JDDF schema file")
                .last(true)
                .required(true),
        );

    let app = target::rust::Target::args(app);

    let matches = app.get_matches();

    // Parse out the input schema, and ensure it is valid.
    let input = matches.value_of("INPUT").unwrap();
    let file = File::open(input)?;
    let serde_schema: SerdeSchema = serde_json::from_reader(file)?;
    let schema: Schema = serde_schema
        .try_into()
        .map_err(|err| format_err!("{:?}", err))?;

    let rust = target::rust::Target::from_args(&matches)?;

    if let Some(rust) = rust {
        rust.codegen(&schema)?;
    }

    Ok(())
}
