mod root_name;

use std::io::Read;
use anyhow::{format_err, Context, Result};
use clap::{crate_version, load_yaml, App};
use jtd::{Schema, SerdeSchema};
use serde::Serialize;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from(cli_yaml).version(crate_version!()).get_matches();

    let mut log: Box<dyn Log> = match matches.value_of("log-format").unwrap() {
        "pretty" => Box::new(PrettyLog()),
        "minimal" => Box::new(MinimalLog()),
        "json" => Box::new(JsonLog(BTreeMap::new())),
        _ => unreachable!(),
    };

    let input = matches.value_of("schema").unwrap();

    // Determine the desired root name to pass to jtd_codegen. If the user has
    // supplied root-name, we'll use that. Otherwise, we'll infer a desired root
    // name from the name of the input file.
    let root_name =
        root_name::root_name_from_input_name(matches.value_of("root-name").unwrap_or(input))
            .to_owned();

    // Open, parse, and validate the input schema.
    let input_reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(input).with_context(|| "Failed to open input file")?),
    };

    let serde_schema: SerdeSchema =
        serde_json::from_reader(input_reader).with_context(|| "Failed to parse input as JSON")?;

    let schema: Schema = serde_schema
        .try_into()
        .map_err(|err| format_err!("{:?}", err))
        .with_context(|| "Failed to validate input schema")?;

    // Generate code for all enabled targets.

    if let Some(out_dir) = matches.value_of("csharp-system-text-out") {
        log.start("C# + System.Text.Json", out_dir);

        let namespace = matches
            .value_of("csharp-system-text-namespace")
            .unwrap()
            .to_owned();

        let target = jtd_codegen_target_csharp_system_text::Target::new(namespace);

        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir))
                .with_context(|| "Failed to generate C# + System.Text.Json code")?;

        log.finish("C# + System.Text.Json", &codegen_info);
    }

    if let Some(out_dir) = matches.value_of("go-out") {
        log.start("Go", out_dir);

        let package = matches
            .value_of("go-package")
            .unwrap()
            .to_owned();

        let target = jtd_codegen_target_go::Target::new(package);

        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir))
                .with_context(|| "Failed to generate Go code")?;

        log.finish("Go", &codegen_info);
    }

    if let Some(out_dir) = matches.value_of("java-jackson-out") {
        log.start("Java + Jackson", out_dir);

        let package = matches
            .value_of("java-jackson-package")
            .unwrap()
            .to_owned();

        let target = jtd_codegen_target_java_jackson::Target::new(package);

        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir))
                .with_context(|| "Failed to generate Java + Jackson code")?;

        log.finish("Java + Jackson", &codegen_info);
    }

    if let Some(out_dir) = matches.value_of("python-out") {
        log.start("Python", out_dir);

        let target = jtd_codegen_target_python::Target::new();

        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir))
                .with_context(|| "Failed to generate Python code")?;

        log.finish("Python", &codegen_info);
    }

    if let Some(out_dir) = matches.value_of("typescript-out") {
        log.start("TypeScript", out_dir);

        let target = jtd_codegen_target_typescript::Target::new();

        let codegen_info =
            jtd_codegen::codegen(&target, root_name.clone(), &schema, &Path::new(out_dir))
                .with_context(|| "Failed to generate TypeScript code")?;

        log.finish("TypeScript", &codegen_info);
    }

    log.flush();
    Ok(())
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
            "📝 Writing {} code to: {}",
            target.green().bold(),
            out_dir.bold()
        );
    }

    fn finish(&mut self, target: &str, info: &jtd_codegen::codegen::CodegenInfo) {
        use colored::*;

        println!("📦 Generated {} code.", target.green().bold());
        println!(
            "📦     Root schema converted into type: {}",
            info.root_name.bold()
        );
        for (definition_name, type_name) in &info.definition_names {
            println!(
                "📦     Definition {} converted into type: {}",
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
