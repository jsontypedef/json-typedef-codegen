use inflector::string::singularize::to_singular;
use inflector::Inflector;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn root_name_from_input_name(input: &str) -> &str {
    Path::new(input)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .trim_end_matches(".jtd")
}

pub struct State<'a, Data> {
    singularize: bool,
    must_emit: bool,
    path: Vec<&'a str>,
    data: Data,
}

impl<'a, Data> State<'a, Data> {
    pub fn new(data: Data) -> Self {
        Self {
            path: Vec::new(),
            singularize: false,
            must_emit: false,
            data,
        }
    }

    pub fn with_singularize<T>(&mut self, f: &dyn Fn(&mut Self) -> T) -> T {
        self.singularize = true;
        let t = f(self);
        self.singularize = false;

        t
    }

    pub fn with_must_emit<T>(&mut self, f: &dyn Fn(&mut Self) -> T) -> T {
        self.must_emit = true;
        let t = f(self);
        self.must_emit = false;

        t
    }

    pub fn with_path_segment<T>(&mut self, path_segment: &'a str, f: &dyn Fn(&mut Self) -> T) -> T {
        self.singularize = false;
        self.must_emit = false;
        self.path.push(path_segment);
        let t = f(self);
        self.path.pop();

        t
    }

    pub fn name(&self) -> String {
        let out = self.path.join("_").to_pascal_case();
        if self.singularize {
            to_singular(&out)
        } else {
            out
        }
    }

    pub fn must_emit(&self) -> bool {
        self.must_emit
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Data {
        &mut self.data
    }
}

pub trait Namespace<T> {
    fn insert_name(&mut self, reserved: &BTreeSet<String>, name: String, value: T) -> String;
}

type BTreeNamespace<T> = BTreeMap<String, T>;

impl<T> Namespace<T> for BTreeNamespace<T> {
    fn insert_name(&mut self, reserved: &BTreeSet<String>, name: String, value: T) -> String {
        if !reserved.contains(&name) && !self.contains_key(&name) {
            self.insert(name.clone(), value);
            return name;
        }

        let mut i = 1;
        loop {
            let candidate_name = format!("{}{}", name, i);
            if !reserved.contains(&candidate_name) && !self.contains_key(&candidate_name) {
                self.insert(candidate_name.clone(), value);
                return candidate_name;
            }

            i += 1;
        }
    }
}
