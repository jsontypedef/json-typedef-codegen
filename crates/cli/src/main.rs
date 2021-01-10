mod root_name;

use std::fs::File;
use std::path::Path;
use std::convert::TryInto;
use jtd::{Schema, SerdeSchema};
use clap::{crate_version, load_yaml, App};

fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from(cli_yaml).version(crate_version!()).get_matches();

    let input = matches.value_of("SCHEMA").unwrap();

    let root_name = root_name::root_name_from_input_name(input).to_owned();

    // TODO: Error handling here
    let input_file = File::open(input).unwrap();
    let serde_schema: SerdeSchema = serde_json::from_reader(input_file).unwrap();
    let schema: Schema = serde_schema.try_into().unwrap();

    if let Some(out_dir) = matches.value_of("csharp-system-text-out") {
        eprintln!("C# + System.Text.Json: generating code to: {}", out_dir);

        let namespace = matches
            .value_of("csharp-system-text-namespace")
            .unwrap()
            .to_owned();

        let target = jtd_codegen_target_csharp_system_text::Target::new(namespace);

        // Error handling
        let root_name = jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir)).unwrap();

        eprintln!("C# + System.Text.Json: code generation completed successfully");
        eprintln!("C# + System.Text.Json: root type name: {}", root_name);
    }
}
