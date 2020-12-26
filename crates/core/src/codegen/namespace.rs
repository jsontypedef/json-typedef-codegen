use std::collections::BTreeSet;

pub struct Namespace(BTreeSet<String>);

impl Namespace {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn get(&mut self, name: String) -> String {
        if !self.0.contains(&name) {
            self.0.insert(name.clone());
            return name;
        }

        let mut i = 0;
        loop {
            let key = format!("{}{}", name, i);
            if !self.0.contains(&key) {
                self.0.insert(key.clone());
                return key;
            }

            i += 1;
        }
    }
}
