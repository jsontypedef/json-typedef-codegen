use crate::target::inflect::decompose;
use teeter_inflector::string::singularize::to_singular;

pub struct NamePath {
    elems: Vec<String>,
    lengths: Vec<usize>,
}

impl NamePath {
    pub fn new() -> Self {
        Self { elems: vec![], lengths: vec![] }
    }

    pub fn push(&mut self, s: &str) {
        let parts = decompose(s);
        self.lengths.push(parts.len());
        self.elems.extend(parts);
    }

    pub fn pop(&mut self) {
        let n = self.lengths.pop().unwrap();
        self.elems.truncate(self.elems.len() - n);
    }

    pub fn singularize(&mut self) {
        let last = self.elems.last_mut().unwrap();
        *last = to_singular(&last);
    }

    pub fn get(&self) -> &[String] {
        &self.elems
    }
}
