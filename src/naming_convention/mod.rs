use inflector::Inflector;
use std::collections::BTreeSet;

pub struct NamingConvention {
    default_separator_style: SeparatorStyle,
    keywords: BTreeSet<String>,
    default_name: String,
}

pub enum SeparatorStyle {
    SnakeCase,
    CamelCase,
    PascalCase,
    ScreamingSnakeCase,
}

impl NamingConvention {
    pub fn new(
        default_separator_style: SeparatorStyle,
        keywords: BTreeSet<String>,
        default_name: String,
    ) -> Self {
        Self {
            default_separator_style,
            keywords,
            default_name,
        }
    }

    pub fn get(&self, s: &[&str]) -> String {
        self.get_with_style(false, &self.default_separator_style, s)
    }

    pub fn get_with_singular(&self, singular: bool, s: &[&str]) -> String {
        self.get_with_style(singular, &self.default_separator_style, s)
    }

    pub fn get_with_style(&self, singular: bool, style: &SeparatorStyle, s: &[&str]) -> String {
        let base_name = Self::apply_separator_style(style, &s.join("_"));
        let base_name = if singular {
            base_name.to_singular()
        } else {
            base_name
        };

        self.mangle(base_name)
    }

    fn mangle(&self, name: String) -> String {
        // First, retain only ASCII.
        let name: String = name.chars().filter(char::is_ascii).collect();

        // If the name is empty or starts with a number, then we cannot mangle
        // it into a good name. Fall back to the default name.
        if name.is_empty() || !name.chars().nth(0).unwrap().is_ascii_alphabetic() {
            return Self::apply_separator_style(&self.default_separator_style, &self.default_name);
        }

        // If the resulting name is a keyword, we can "dodge" that keyword by
        // manipulating the name.
        if self.keywords.contains(&name) {
            // If we want to support other keyword dodging strategies, this is
            // the place to update.
            return format!("{}_", name);
        }

        name
    }

    fn apply_separator_style(style: &SeparatorStyle, name: &str) -> String {
        match style {
            SeparatorStyle::CamelCase => name.to_camel_case(),
            SeparatorStyle::SnakeCase => name.to_snake_case(),
            SeparatorStyle::PascalCase => name.to_pascal_case(),
            SeparatorStyle::ScreamingSnakeCase => name.to_screaming_snake_case(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangling() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::SnakeCase,
            vec!["key_word".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        assert_eq!("foo", convention.get(&["foo"]));
        assert_eq!("fo_o", convention.get(&["foço"]));
        assert_eq!("foo", convention.get(&[" foo"]));
        assert_eq!("default", convention.get(&[""]));
        assert_eq!("default", convention.get(&["\t"]));
        assert_eq!("default", convention.get(&["1"]));
        assert_eq!("default", convention.get(&["1foo"]));
        assert_eq!("keyword", convention.get(&["keyword"]));
        assert_eq!("key_word_", convention.get(&["key_word_"]));
        assert_eq!("key_word_", convention.get(&["key", "word_"]));
    }

    #[test]
    fn test_get_with_style() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::SnakeCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        assert_eq!("foo_bar", convention.get(&["foo", "bar"]));

        assert_eq!(
            "fooBar",
            convention.get_with_style(false, &SeparatorStyle::CamelCase, &["foo", "bar"])
        );

        assert_eq!(
            "fooBars",
            convention.get_with_style(false, &SeparatorStyle::CamelCase, &["foo", "bars"])
        );

        assert_eq!(
            "fooBar",
            convention.get_with_style(true, &SeparatorStyle::CamelCase, &["foo", "bars"])
        );
    }

    #[test]
    fn test_snake_case() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::SnakeCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        test_convention(
            &convention,
            vec![
                (vec![], "default"),
                (vec![""], "default"),
                (vec!["foo"], "foo"),
                (vec!["foo", "bar"], "foo_bar"),
                (vec![" ", "foo", " ", "bar"], "foo_bar"),
                (vec!["keyword"], "keyword_"),
            ],
        );
    }

    #[test]
    fn test_camel_case() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::CamelCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        test_convention(
            &convention,
            vec![
                (vec![], "default"),
                (vec![""], "default"),
                (vec!["foo"], "foo"),
                (vec!["foo", "bar"], "fooBar"),
                (vec![" ", "foo", " ", "bar"], "fooBar"),
                (vec!["keyword"], "keyword_"),
            ],
        );
    }

    #[test]
    fn test_pascal_case() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::PascalCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        test_convention(
            &convention,
            vec![
                (vec![], "Default"),
                (vec![""], "Default"),
                (vec!["foo"], "Foo"),
                (vec!["foo", "bar"], "FooBar"),
                (vec![" ", "foo", " ", "bar"], "FooBar"),
                (vec!["keyword"], "Keyword"),
            ],
        );
    }

    #[test]
    fn test_screaming_snake_case() {
        let convention = NamingConvention::new(
            super::SeparatorStyle::ScreamingSnakeCase,
            vec!["keyword".to_owned()].into_iter().collect(),
            "default".to_owned(),
        );

        test_convention(
            &convention,
            vec![
                (vec![], "DEFAULT"),
                (vec![""], "DEFAULT"),
                (vec!["foo"], "FOO"),
                (vec!["foo", "bar"], "FOO_BAR"),
                (vec![" ", "foo", " ", "bar"], "FOO_BAR"),
                (vec!["keyword"], "KEYWORD"),
            ],
        );
    }

    fn test_convention(
        convention: &NamingConvention,
        test_cases: Vec<(Vec<&'static str>, &'static str)>,
    ) {
        for test_case in test_cases {
            assert_eq!(convention.get(&test_case.0), test_case.1);
        }
    }
}
