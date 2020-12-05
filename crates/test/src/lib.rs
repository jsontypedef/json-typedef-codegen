use jtd::{Schema, SerdeSchema, Validator};
use jtd_codegen::Target;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde_json::{Deserializer, Value};
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn assert_common_test_cases<T: Target>(target_crate_base_dir: &str, target: &T) {
    // todo: do other test cases
    let schema = File::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("schemas")
            .join(Path::new("basic_properties").with_extension("jtd.json")),
    )
    .expect("open schema file");

    let schema: SerdeSchema = serde_json::from_reader(schema).expect("deserialize schema");
    let schema: Schema = schema.try_into().expect("validate schema");

    // todo: verifying stability only makes sense for common test cases, custom
    // test cases will need to provide test case name somehow.
    let tempdir = assert_roundtrip(target_crate_base_dir, target, &schema, 8927);
    assert_stable(target_crate_base_dir, "basic_properties", tempdir);
}

fn assert_roundtrip<T: Target>(
    target_crate_base_dir: &str,
    target: &T,
    schema: &Schema,
    seed: u64,
) -> tempfile::TempDir {
    // The dir where we'll do all of our work.
    let tempdir = tempfile::tempdir().expect("create temp dir");

    // The dir where we'll generate code into.
    let codegen_dir = tempdir.path().join("gen");
    fs::create_dir(&codegen_dir).expect("create gen dir");

    // Generate code into codegen_dir.
    let main = jtd_codegen::codegen(target, "Root".to_owned(), &schema, &codegen_dir)
        .expect("generate code");

    // Copy over the target crate's docker data into the temp dir.
    let crate_docker_dir = Path::new(target_crate_base_dir).join("docker");
    for entry in fs::read_dir(crate_docker_dir).expect("read crate docker dir") {
        let entry = entry.expect("read crate docker dir entry");

        fs::copy(
            entry.path(),
            tempdir.path().join(
                entry
                    .path()
                    .file_name()
                    .expect("crate docker dir entry file name"),
            ),
        )
        .expect("copy crate docker dir entry to temp dir");
    }

    // Fuzz some data against the schema.
    let mut rng = Pcg32::seed_from_u64(seed);
    let mut input = Vec::new();
    for _ in 0..1000 {
        writeln!(input, "{}", jtd_fuzz::fuzz(schema, &mut rng)).unwrap();
    }

    // Build the docker container. We pipe stdout so we can get back the image
    // docker created for a later invocation with docker run.
    //
    // We let docker build inherit out stderr, that way if the build fails
    // docker will have outputted a log to stderr, which the Rust test runner
    // outputs on test failures.
    let mut docker_build = Command::new("docker")
        .arg("build")
        .arg("--quiet")
        .arg("--build-arg")
        .arg(format!("MAIN={}", main.expr))
        .arg(tempdir.path())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn docker build");

    // Ensure docker build succeeds.
    assert!(
        docker_build.wait().expect("wait docker build").success(),
        "docker build failed"
    );

    // Read the outputted build image.
    let mut image = String::new();
    docker_build
        .stdout
        .unwrap()
        .read_to_string(&mut image)
        .expect("read docker build stdout");

    // Run the docker container, with the input piped in.
    let mut docker_run = Command::new("docker")
        .arg("run")
        .arg("--interactive")
        .arg(image.trim_end())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn docker run");

    // Send the input JSON in.
    docker_run
        .stdin
        .unwrap()
        .write_all(&input)
        .expect("write docker run stdin");

    // Get the output JSON out.
    let mut output = String::new();
    docker_run
        .stdout
        .unwrap()
        .read_to_string(&mut output)
        .expect("read docker run stdout");

    // Ensure the output is a sequence of JSON values satisfying the input
    // schema.
    let validator = Validator {
        max_depth: None,
        max_errors: None,
    };

    for value in Deserializer::from_str(&output).into_iter() {
        let value: Value = value.expect("parse docker run output");
        let errors = validator.validate(schema, &value).unwrap();

        assert_eq!(
            errors.len(),
            0,
            "validation error from output: {:?} {:?}",
            value,
            errors,
        );
    }

    // Return the tempdir where we generated data into.
    tempdir
}

fn assert_stable(target_crate_base_dir: &str, test_case_name: &str, tempdir: tempfile::TempDir) {
    let output_dir = tempdir.path().join("gen");
    let reference_dir = Path::new(target_crate_base_dir)
        .join("output")
        .join(test_case_name);

    // Just a sanity check.
    assert!(output_dir.exists(), "tempdir has no gen subdirectory");

    // If there is no reference dir for this crate, then we will make the
    // tempdir be the new reference output.
    //
    // In effect, this means jtd-codegen developers can "opt into" a change in
    // stability of output by deleting reference dirs that are no longer
    // relevant.
    //
    // An improvement to this code would be to detect that we are in CI, and
    // panic if we are trying to create a reference output in CI. Creating new
    // reference output should only happen in local development.
    if !reference_dir.exists() {
        // Create the reference dir.
        fs::create_dir(&reference_dir).expect("create reference dir");

        // Copy each file in output_dir into reference_dir.
        for entry in output_dir.read_dir().expect("read output dir") {
            let entry = entry.expect("read output entry");

            fs::copy(
                entry.path(),
                reference_dir.join(entry.path().file_name().unwrap()),
            )
            .expect("copy output file to reference dir");
        }
    }

    // Ensure that the set of files in the output is the same set of files in
    // the reference.
    let output_files: BTreeSet<_> = output_dir
        .read_dir()
        .expect("read output dir")
        .map(|entry| {
            entry
                .expect("read output entry")
                .path()
                .file_name()
                .unwrap()
                .to_owned()
        })
        .collect();

    let reference_files: BTreeSet<_> = reference_dir
        .read_dir()
        .expect("read reference dir")
        .map(|entry| {
            entry
                .expect("read output entry")
                .path()
                .file_name()
                .unwrap()
                .to_owned()
        })
        .collect();

    assert_eq!(
        output_files, reference_files,
        "output and reference contain different file sets"
    );

    // Check that each output file has the same contents as the reference
    // output.
    for file in output_files {
        let output_file = fs::read_to_string(output_dir.join(&file)).expect("read output file");
        let reference_file =
            fs::read_to_string(reference_dir.join(&file)).expect("read output file");

        assert_eq!(
            output_file, reference_file,
            "output and reference file differ: {:?}",
            file
        );
    }
}
