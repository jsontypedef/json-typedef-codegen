use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "bar")]
    pub bar: String,

    #[serde(rename = "baz")]
    pub baz: Vec<bool>,

    #[serde(rename = "foo")]
    pub foo: bool,

    #[serde(rename = "quux")]
    pub quux: Vec<bool>,
}
