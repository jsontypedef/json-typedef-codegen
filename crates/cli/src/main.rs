mod root_name;

use clap::{crate_version, load_yaml, App};
use jtd::{Schema, SerdeSchema};
use serde::Serialize;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;

fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from(cli_yaml).version(crate_version!()).get_matches();

    let mut log: Box<dyn Log> = match matches.value_of("log-format").unwrap() {
        "pretty" => Box::new(PrettyLog()),
        "minimal" => Box::new(MinimalLog()),
        "json" => Box::new(JsonLog(BTreeMap::new())),
        _ => unreachable!(),
    };

    let input = matches.value_of("SCHEMA").unwrap();

    // TODO: support overriding root name by cli argument
    let root_name = root_name::root_name_from_input_name(input).to_owned();

    // TODO: Error handling here
    let input_file = File::open(input).unwrap();
    let serde_schema: SerdeSchema = serde_json::from_reader(input_file).unwrap();
    let schema: Schema = serde_schema.try_into().unwrap();

    if let Some(out_dir) = matches.value_of("csharp-system-text-out") {
        log.start("C# + System.Text.Json", out_dir);

        let namespace = matches
            .value_of("csharp-system-text-namespace")
            .unwrap()
            .to_owned();

        let target = jtd_codegen_target_csharp_system_text::Target::new(namespace);

        // Error handling
        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir)).unwrap();

        log.finish("C# + System.Text.Json", &codegen_info);
    }

    log.flush();
}

trait Log {
    fn start(&mut self, target: &str, out_dir: &str);
    fn finish(&mut self, target: &str, info: &jtd_codegen::codegen::CodegenInfo);
    fn flush(&mut self);
}

struct PrettyLog();
impl Log for PrettyLog {
    fn start(&mut self, target: &str, out_dir: &str) {
        use colored::*;

        println!(
            "✍️  Writing {} code to: {}",
            target.green().bold(),
            out_dir.bold()
        );
    }

    fn finish(&mut self, target: &str, info: &jtd_codegen::codegen::CodegenInfo) {
        use colored::*;

        println!("📦 Generated {} code.", target.green().bold());
        println!(
            "📦\tRoot schema converted into type: {}",
            info.root_name.bold()
        );
        for (definition_name, type_name) in &info.definition_names {
            println!(
                "📦\tDefinition {} converted into type: {}",
                format!("{:?}", definition_name).bold(),
                type_name.bold()
            );
        }
    }

    fn flush(&mut self) {}
}

struct MinimalLog();
impl Log for MinimalLog {
    fn start(&mut self, target: &str, out_dir: &str) {
        eprintln!("{}: writing to: {}", target, out_dir);
    }

    fn finish(&mut self, target: &str, info: &jtd_codegen::codegen::CodegenInfo) {
        println!("{}: root: {}", target, &info.root_name);
        for (definition_name, type_name) in &info.definition_names {
            println!("{}: definition: {}: {}", target, definition_name, type_name);
        }
    }

    fn flush(&mut self) {}
}

struct JsonLog(BTreeMap<String, TargetEntry>);

#[derive(Serialize)]
struct TargetEntry {
    out_dir: String,
    root_name: String,
    definition_names: BTreeMap<String, String>,
}

impl Log for JsonLog {
    fn start(&mut self, target: &str, out_dir: &str) {
        self.0.insert(
            target.to_owned(),
            TargetEntry {
                out_dir: out_dir.to_owned(),
                root_name: "".to_owned(),
                definition_names: BTreeMap::new(),
            },
        );
    }

    fn finish(&mut self, target: &str, info: &jtd_codegen::codegen::CodegenInfo) {
        let mut entry = self.0.get_mut(target).unwrap();

        entry.root_name = info.root_name.clone();
        entry.definition_names = info.definition_names.clone();
    }

    fn flush(&mut self) {
        println!("{}", serde_json::to_string(&self.0).unwrap());
    }
}
