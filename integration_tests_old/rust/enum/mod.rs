use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub enum Enum {

    #[serde(rename = "bar")]
    Bar,

    #[serde(rename = "baz")]
    Baz,

    #[serde(rename = "foo")]
    Foo,
}

