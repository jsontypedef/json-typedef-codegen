use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


pub type Elements = Vec<Element>;

pub type Empty = serde_json::Value;

pub type Values = HashMap<String, Value>;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "foo")]
pub enum Discriminator {

    #[serde(rename = "bar")]
    Bar(DiscriminatorBar),

    #[serde(rename = "baz")]
    Baz(DiscriminatorBaz),
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Enum {

    #[serde(rename = "bar")]
    Bar,

    #[serde(rename = "baz")]
    Baz,

    #[serde(rename = "foo")]
    Foo,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscriminatorBar {

    #[serde(rename = "barThing")]
    bar_thing: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscriminatorBaz {

    #[serde(rename = "bazThing")]
    baz_thing: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {

    #[serde(rename = "elementThing")]
    element_thing: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gamut {

    #[serde(rename = "discriminator")]
    discriminator: Discriminator,

    #[serde(rename = "elements")]
    elements: Elements,

    #[serde(rename = "empty")]
    empty: Empty,

    #[serde(rename = "enum")]
    enum: Enum,

    #[serde(rename = "type")]
    type: Type,

    #[serde(rename = "values")]
    values: Values,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Type {

    #[serde(rename = "boolean")]
    boolean: bool,

    #[serde(rename = "float32")]
    float_32: f32,

    #[serde(rename = "float64")]
    float_64: f64,

    #[serde(rename = "int16")]
    int_16: i16,

    #[serde(rename = "int32")]
    int_32: i32,

    #[serde(rename = "int8")]
    int_8: i8,

    #[serde(rename = "string")]
    string: String,

    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,

    #[serde(rename = "uint16")]
    uint_16: u16,

    #[serde(rename = "uint32")]
    uint_32: u32,

    #[serde(rename = "uint8")]
    uint_8: u8,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value {

    #[serde(rename = "valueThing")]
    value_thing: serde_json::Value,
}

