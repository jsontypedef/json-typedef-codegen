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

// todo: remove this, use file naming convention instead
pub struct AppendingInflector<I> {
    suffix: String,
    inflector: I,
}

impl<I> AppendingInflector<I> {
    pub fn new(suffix: String, inflector: I) -> Self {
        Self { suffix, inflector }
    }
}

impl<I: Inflector> Inflector for AppendingInflector<I> {
    fn inflect(&self, name_parts: &[String]) -> String {
        format!("{}{}", self.inflector.inflect(name_parts), self.suffix)
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
        self.case.to_case(&name_parts.join("_"))
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
            .to_case(name_parts.last().expect("TailInflector: empty name_parts"))
    }
}

pub enum Case {
    SnakeCase,
    CamelCase,
    PascalCase,
    ScreamingSnakeCase,
}

impl Case {
    fn to_case(&self, s: &str) -> String {
        use teeter_inflector::cases::camelcase::to_camel_case;
        use teeter_inflector::cases::pascalcase::to_pascal_case;
        use teeter_inflector::cases::screamingsnakecase::to_screaming_snake_case;
        use teeter_inflector::cases::snakecase::to_snake_case;

        match self {
            Self::SnakeCase => to_snake_case(s),
            Self::CamelCase => to_camel_case(s),
            Self::PascalCase => to_pascal_case(s),
            Self::ScreamingSnakeCase => to_screaming_snake_case(s),
        }
    }
}
