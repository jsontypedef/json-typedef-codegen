use std::collections::BTreeSet;

pub trait Inflector {
    fn inflect(&self, name_parts: &[String]) -> String;
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
    fn inflect(&self, name_parts: &[String]) -> String {
        let raw_name = self.inflector.inflect(name_parts);

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
    fn inflect(&self, name_parts: &[String]) -> String {
        self.case.inflect(name_parts)
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
    fn inflect(&self, name_parts: &[String]) -> String {
        self.case
            .inflect(&[name_parts.last().expect("TailInflector: empty name_parts").clone()])
    }
}

// pub enum Case {
//     SnakeCase,
//     CamelCase,
//     PascalCase,
//     ScreamingSnakeCase,
// }

// impl Case {
//     fn to_case(&self, s: &str) -> String {
//         use teeter_inflector::cases::camelcase::to_camel_case;
//         use teeter_inflector::cases::pascalcase::to_pascal_case;
//         use teeter_inflector::cases::screamingsnakecase::to_screaming_snake_case;
//         use teeter_inflector::cases::snakecase::to_snake_case;

//         match self {
//             Self::SnakeCase => to_snake_case(s),
//             Self::CamelCase => to_camel_case(s),
//             Self::PascalCase => to_pascal_case(s),
//             Self::ScreamingSnakeCase => to_screaming_snake_case(s),
//         }
//     }
// }

pub fn decompose(s: &str) -> Vec<String> {
    let mut out: Vec<Vec<char>> = vec![vec![]];
    for c in s.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            out.push(vec![]);
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
}

impl Case {
    pub fn new(
        first_capitalization: CaseCapitalization,
        rest_capitalization: CaseCapitalization,
        delimiter: Option<char>,
    ) -> Self {
        Self {
            first_capitalization,
            rest_capitalization,
            delimiter,
        }
    }

    pub fn camel_case() -> Self {
        Self::new(CaseCapitalization::None, CaseCapitalization::Initial, None)
    }

    pub fn pascal_case() -> Self {
        Self::new(CaseCapitalization::Initial, CaseCapitalization::Initial, None)
    }

    pub fn snake_case() -> Self {
        Self::new(CaseCapitalization::None, CaseCapitalization::None, Some('_'))
    }

    pub fn screaming_snake_case() -> Self {
        Self::new(CaseCapitalization::All, CaseCapitalization::All, Some('_'))
    }

    pub fn inflect(&self, words: &[String]) -> String {
        if words.is_empty() {
            return "".to_owned();
        }

        let parts: Vec<_> = words
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                if i == 0 {
                    self.first_capitalization.inflect(word)
                } else {
                    self.rest_capitalization.inflect(word)
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
        assert_eq!(vec!["foo", "bar"], decompose("FOO BAR"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO-BAR"));
        assert_eq!(vec!["foo", "bar"], decompose("FOO_BAR"));

        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo barBaz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("fooBar-baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo-bar_baz"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("foo_bar BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO BAR-BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO-BAR_BAZ"));
        assert_eq!(vec!["foo", "bar", "baz"], decompose("FOO_BAR baz"));
    }

    #[test]
    fn test_camel_case() {
        assert_eq!("", Case::camel_case().inflect(&[]));

        assert_eq!("foo", Case::camel_case().inflect(&["foo".to_owned()]));
        assert_eq!("fooBar", Case::camel_case().inflect(&["foo".to_owned(), "bar".to_owned()]));
        assert_eq!(
            "fooBarBaz",
            Case::camel_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!("", Case::pascal_case().inflect(&[]));

        assert_eq!("Foo", Case::pascal_case().inflect(&["foo".to_owned()]));
        assert_eq!("FooBar", Case::pascal_case().inflect(&["foo".to_owned(), "bar".to_owned()]));
        assert_eq!(
            "FooBarBaz",
            Case::pascal_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_snake_case() {
        assert_eq!("", Case::snake_case().inflect(&[]));

        assert_eq!("foo", Case::snake_case().inflect(&["foo".to_owned()]));
        assert_eq!("foo_bar", Case::snake_case().inflect(&["foo".to_owned(), "bar".to_owned()]));
        assert_eq!(
            "foo_bar_baz",
            Case::snake_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }

    #[test]
    fn test_screaming_snake_case() {
        assert_eq!("", Case::screaming_snake_case().inflect(&[]));

        assert_eq!("FOO", Case::screaming_snake_case().inflect(&["foo".to_owned()]));
        assert_eq!("FOO_BAR", Case::screaming_snake_case().inflect(&["foo".to_owned(), "bar".to_owned()]));
        assert_eq!(
            "FOO_BAR_BAZ",
            Case::screaming_snake_case().inflect(&["foo".to_owned(), "bar".to_owned(), "baz".to_owned()])
        );
    }
}
