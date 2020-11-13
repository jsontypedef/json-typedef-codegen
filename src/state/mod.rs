use crate::naming_convention::NamingConvention;

pub struct State<T> {
    path: Vec<String>,
    singularize: bool,
    must_emit: bool,
    priority: usize,
    naming_convention: NamingConvention,
    data: T,
}

impl<T> State<T> {
    pub fn new(naming_convention: NamingConvention, data: T) -> Self {
        Self {
            path: Vec::new(),
            singularize: false,
            must_emit: false,
            priority: 0,
            naming_convention,
            data,
        }
    }

    pub fn with_singularize<U, F>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
    {
        let singularize = self.singularize;

        self.singularize = true;
        let t = f(self);
        self.singularize = singularize;

        t
    }

    pub fn with_must_emit<U, F>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
    {
        let must_emit = self.must_emit;

        self.must_emit = true;
        let t = f(self);
        self.must_emit = must_emit;

        t
    }

    pub fn with_priority<U, F>(&mut self, priority: usize, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
    {
        let prev_priority = self.priority;

        self.priority = priority;
        let t = f(self);
        self.priority = prev_priority;

        t
    }

    pub fn with_path_segment<U, V, F>(&mut self, path_segment: V, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
        V: Into<String>,
    {
        let singularize = self.singularize;
        let must_emit = self.must_emit;
        let priority = self.priority;

        self.path.push(path_segment.into());
        self.singularize = false;
        self.must_emit = false;
        self.priority += 1;

        let t = f(self);

        self.priority = priority;
        self.must_emit = must_emit;
        self.singularize = singularize;
        self.path.pop();

        t
    }

    pub fn name(&self) -> String {
        let path: Vec<&str> = self.path.iter().map(AsRef::as_ref).collect();
        self.naming_convention
            .get_with_singular(self.singularize, &path)
    }

    pub fn must_emit(&self) -> bool {
        self.must_emit
    }

    pub fn priority(&self) -> usize {
        self.priority
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        use crate::naming_convention::SeparatorStyle;

        let convention = NamingConvention::new(
            SeparatorStyle::SnakeCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        let state = State::new(convention, vec!["foo".to_owned()]);

        // Test initial state.
        assert_eq!(state.name(), "default".to_owned());
        assert_eq!(state.must_emit(), false);
        assert_eq!(state.priority(), 0);
        assert_eq!(state.into_data(), &["foo".to_owned()]);

        let convention = NamingConvention::new(
            SeparatorStyle::SnakeCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        let mut state = State::new(convention, vec!["foo".to_owned()]);

        // Test that the with_* functions give back their return values.
        assert_eq!(42, state.with_singularize(|_| 42));
        assert_eq!(42, state.with_must_emit(|_| 42));
        assert_eq!(42, state.with_priority(0, |_| 42));
        assert_eq!(42, state.with_path_segment("".to_owned(), |_| 42));

        // Test with_path_segment changing name() and priority()
        state.with_path_segment("foo", |state| {
            assert_eq!(1, state.priority());
            assert_eq!("foo".to_owned(), state.name());

            state.with_path_segment("bar", |state| {
                assert_eq!(2, state.priority());
                assert_eq!("foo_bar".to_owned(), state.name());

                state.with_path_segment("baz", |state| {
                    assert_eq!(3, state.priority());
                    assert_eq!("foo_bar_baz".to_owned(), state.name());
                });
            });
        });

        // Test with_priority interacting with with_path_segment
        state.with_priority(3, |state| {
            assert_eq!(3, state.priority());

            state.with_path_segment("foo", |state| {
                assert_eq!(4, state.priority());
            });
        });

        // Test with_singularize interacting with with_path_segment
        state.with_path_segment("foos", |state| {
            assert_eq!("foos".to_owned(), state.name());

            state.with_singularize(|state| {
                assert_eq!("foo".to_owned(), state.name());

                state.with_path_segment("bars", |state| {
                    // At the time of writing, this behavior -- wherein only the
                    // last segment of a name can ever be singularized -- is
                    // considered acceptable.
                    assert_eq!("foos_bars".to_owned(), state.name());
                });
            });
        });

        // Test with_must_emit interacting with with_path_segment
        state.with_path_segment("foo", |state| {
            assert_eq!(false, state.must_emit());

            state.with_must_emit(|state| {
                assert_eq!(true, state.must_emit());

                state.with_path_segment("bar", |state| {
                    assert_eq!(false, state.must_emit());
                });
            });
        });
    }
}
