// Code generated by jtd-codegen for Rust v0.2.1

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Root {
    #[serde(rename = "Bar")]
    Bar,

    #[serde(rename = "Baz")]
    Baz,

    #[serde(rename = "Foo")]
    Foo,
}
