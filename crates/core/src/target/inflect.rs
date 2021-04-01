use std::collections::BTreeSet;

pub trait Inflector {
    fn inflect(&self, words: &[String]) -> String;
}

pub struct KeywordAvoidingInflector<I> {
    keywords: BTreeSet<String>,
    inflector: I,
}

impl<I> KeywordAvoidingInflector<I> {
    pub fn new(keywords: BTreeSet<String>, inflector: I) -> Self {
        Self {
            keywords,
            inflector,
        }
    }
}

impl<I: Inflector> Inflector for KeywordAvoidingInflector<I> {
    fn inflect(&self, words: &[String]) -> String {
        let raw_name = self.inflector.inflect(words);

        if self.keywords.contains(&raw_name) {
            format!("{}_", raw_name)
        } else {
            raw_name
        }
    }
}

pub struct CombiningInflector {
    case: Case,
}

impl CombiningInflector {
    pub fn new(case: Case) -> Self {
        Self { case }
    }
}

impl Inflector for CombiningInflector {
    fn inflect(&self, words: &[String]) -> String {
        self.case.inflect(words)
    }
}

pub struct TailInflector {
    case: Case,
}

impl TailInflector {
    pub fn new(case: Case) -> Self {
        Self { case }
    }
}

impl Inflector for TailInflector {
    fn inflect(&self, words: &[String]) -> String {
        self.case
            .inflect(&[words.last().expect("TailInflector: empty words").clone()])
    }
}

fn decompose(s: &str) -> Vec<String> {
    let mut out: Vec<Vec<char>> = vec![vec![]];
    for c in s.chars() {
        // Non-ASCII alphanumeric characters, such as whitespace, dashes,
        // underscores, or non-ASCII characters, are presumed to always be
        // delimiters.
        if !c.is_ascii_alphanumeric() {
            out.push(vec![]);
            continue;
        }

        // Do not allow a part to start with a digit. Most languages prohibit
        // digits at the beginning of identifiers. Just ignore the digit to make
        // this happen.
        if c.is_ascii_digit() && out.last().unwrap().is_empty() {
            continue;
        }

        if let Some(last_char) = out.last().unwrap().last() {
            if last_char.is_lowercase() && c.is_uppercase() {
                out.push(vec![]);
            }
        }

        out.last_mut().unwrap().push(c);
    }

    out.into_iter()
        .filter(|word| !word.is_empty())
        .map(|chars| chars.into_iter().flat_map(|c| c.to_lowercase()).collect())
        .collect()
}

pub struct Case {
    first_capitalization: CaseCapitalization,
    rest_capitalization: CaseCapitalization,
    delimiter: Option<char>,
    initialisms: BTreeSet<String>,
}

impl Case {
    pub fn new(
        first_capitalization: CaseCapitalization,
        rest_capitalization: CaseCapitalization,
        delimiter: Option<char>,
        initialisms: BTreeSet<String>,
    ) -> Self {
        Self {
            first_capitalization,
            rest_capitalization,
            delimiter,
            initialisms,
        }
    }

    pub fn camel_case() -> Self {
        Self::new(
            CaseCapitalization::None,
            CaseCapitalization::Initial,
            None,
            BTreeSet::new(),
        )
    }

    pub fn pascal_case() -> Self {
        Self::new(
            CaseCapitalization::Initial,
            CaseCapitalization::Initial,
            None,
            BTreeSet::new(),
        )
    }

    pub fn pascal_case_with_initialisms(initialisms: BTreeSet<String>) -> Self {
        Self::new(
            CaseCapitalization::Initial,
            CaseCapitalization::Initial,
            None,
            initialisms,
        )
    }

    pub fn snake_case() -> Self {
        Self::new(
            CaseCapitalization::None,
            CaseCapitalization::None,
            Some('_'),
            BTreeSet::new(),
        )
    }

    pub fn screaming_snake_case() -> Self {
        Self::new(
            CaseCapitalization::All,
            CaseCapitalization::All,
            Some('_'),
            BTreeSet::new(),
        )
    }

    pub fn inflect(&self, words: &[String]) -> String {
        let mut word_parts: Vec<_> = words.into_iter().flat_map(|word| decompose(word)).collect();

        // If after decomposing the word into its parts (and after the
        // associated stripping of non-ASCII alphanumerics) we don't have any
        // words to work with, then inflect a "default name" instead.
        if word_parts.is_empty() {
            word_parts = vec!["default".into(), "name".into()];
        }

        let parts: Vec<_> = word_parts
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                if self.initialisms.contains(&word) {
                    CaseCapitalization::All.inflect(&word)
                } else if i == 0 {
                    self.first_capitalization.inflect(&word)
                } else {
                    self.rest_capitalization.inflect(&word)
                }
            })
            .collect();

        if let Some(delimiter) = self.delimiter {
            parts.join(&delimiter.to_string())
        } else {
            parts.join("")
        }
    }
}

pub enum CaseCapitalization {
    None,
    Initial,
    All,
}

impl CaseCapitalization {
    pub fn inflect(&self, word: &str) -> String {
        if word.is_empty() {
            return "".to_owned();
        }

        match self {
            Self::None => word.to_owned(),
            Self::Initial => {
                let mut c = word.chars();
                c.next().unwrap().to_uppercase().chain(c).collect()
            }
            Self::All => word.chars().flat_map(|c| c.to_uppercase()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose() {
        assert_eq!(Vec::<String>::new(), decompose(""));

        assert_eq!(vec!["foo"], decompose("foo"));
        assert_eq!(vec!["foo"], decompose("foo "));
        assert_eq!(vec!["foo"], decompose(" foo"));
        assert_eq!(vec!["foo"], decompose(" foo "));
        assert_eq!(vec!["foo"], decompose("FOO"));
        assert_eq!(vec!["foo"], decompose(" FOO"));
        assert_eq!(vec!["foo"], decompose("FOO "));
        assert_eq!(vec!["foo"], decompose(" FOO "));

        assert_eq!(vec!["foo", "bar"], decompose("foo bar"));
        assert_eq!(vec!["foo", "bar"], decompose("fooBar"));
        assert_eq!(vec!["foo", "bar"], decompose("foo-bar"));
        assert_eq!(vec!["foo", "bar"], decompose("foo_bar"));
        assert_eq!(vec!["foo", "bar"], decompose("foo::bar"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO BAR"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO-BAR"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO_BAR"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO::BAR"));

        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo barBaz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("fooBar-baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo-bar_baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo_bar::baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo::bar BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO BAR-BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO-BAR_BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO_BAR baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO_BAR::BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO::BAR baz"));
    }

    #[test]
    fn test_camel_case() {
        assert_eq!("defaultName", Case::camel_case().inflect(&[]));

        assert_eq!("foo", Case::camel_case().inflect(&["foo".to_owned()]));
        assert_eq!(
            "fooBar",
            Case::camel_case().inflect(&["foo".to_owned(), "bar".to_owned()])
        );
        assert_eq!(
            "fooBarBaz",
            Case::camel_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!("DefaultName", Case::pascal_case().inflect(&[]));

        assert_eq!("Foo", Case::pascal_case().inflect(&["foo".to_owned()]));
        assert_eq!(
            "FooBar",
            Case::pascal_case().inflect(&["foo".to_owned(), "bar".to_owned()])
        );
        assert_eq!(
            "FooBarBaz",
            Case::pascal_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_snake_case() {
        assert_eq!("default_name", Case::snake_case().inflect(&[]));

        assert_eq!("foo", Case::snake_case().inflect(&["foo".to_owned()]));
        assert_eq!(
            "foo_bar",
            Case::snake_case().inflect(&["foo".to_owned(), "bar".to_owned()])
        );
        assert_eq!(
            "foo_bar_baz",
            Case::snake_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_screaming_snake_case() {
        assert_eq!("DEFAULT_NAME", Case::screaming_snake_case().inflect(&[]));

        assert_eq!(
            "FOO",
            Case::screaming_snake_case().inflect(&["foo".to_owned()])
        );
        assert_eq!(
            "FOO_BAR",
            Case::screaming_snake_case().inflect(&["foo".to_owned(), "bar".to_owned()])
        );
        assert_eq!(
            "FOO_BAR_BAZ",
            Case::screaming_snake_case().inflect(&[
                "foo".to_owned(),
                "bar".to_owned(),
                "baz".to_owned()
            ])
        );
    }
}
