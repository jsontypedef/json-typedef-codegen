use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "bar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<Box<Vec<String>>>,

    #[serde(rename = "baz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baz: Option<Box<bool>>,

    #[serde(rename = "foo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foo: Option<Box<String>>,
}
