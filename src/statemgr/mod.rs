use inflector::string::singularize::to_singular;
use inflector::Inflector;

pub struct StateManager<Data> {
    path: Vec<String>,
    singularize: bool,
    must_emit: bool,
    root_name: String,

    pub data: Data,
}

impl<Data> StateManager<Data> {
    pub fn new(root_name: String, initial_data: Data) -> Self {
        StateManager {
            path: vec![],
            singularize: false,
            must_emit: false,
            root_name,
            data: initial_data,
        }
    }

    pub fn must_emit(&self) -> bool {
        self.must_emit
    }

    pub fn definition_name(&self, definition: &str) -> String {
        definition.to_pascal_case()
    }

    pub fn name(&self) -> String {
        if self.path.is_empty() {
            return self.root_name.to_pascal_case();
        }

        let out = self.path.join("_").to_pascal_case();
        if self.singularize {
            to_singular(&out)
        } else {
            out
        }
    }

    pub fn with_path_segment(
        &mut self,
        segment: String,
        f: &dyn Fn(&mut Self) -> String,
    ) -> String {
        self.path.push(segment);
        let out = self.with_must_emit(false, &|state| state.with_singularize(false, f));
        self.path.pop();

        out
    }

    pub fn with_must_emit(&mut self, must_emit: bool, f: &dyn Fn(&mut Self) -> String) -> String {
        let restore = self.must_emit;

        self.must_emit = must_emit;
        let out = f(self);
        self.must_emit = restore;

        out
    }

    pub fn with_singularize(
        &mut self,
        singularize: bool,
        f: &dyn Fn(&mut Self) -> String,
    ) -> String {
        let restore = self.singularize;

        self.singularize = singularize;
        let out = f(self);
        self.singularize = restore;

        out
    }
}
