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
        let base_name = s.join("_");

        let base_name = if base_name.is_empty() {
            &self.default_name
        } else {
            &base_name
        };

        let base_name = match style {
            SeparatorStyle::CamelCase => base_name.to_camel_case(),
            SeparatorStyle::SnakeCase => base_name.to_snake_case(),
            SeparatorStyle::PascalCase => base_name.to_pascal_case(),
            SeparatorStyle::ScreamingSnakeCase => base_name.to_screaming_snake_case(),
        };

        let base_name = if singular {
            base_name.to_singular()
        } else {
            base_name
        };

        if self.keywords.contains(&base_name) {
            format!("{}_", base_name)
        } else {
            base_name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_with_style() {
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
