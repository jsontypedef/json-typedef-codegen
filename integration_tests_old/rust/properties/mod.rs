use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct D {

    #[serde(rename = "a")]
    a: u32,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Properties {

    #[serde(rename = "a")]
    a: Option<String>,

    #[serde(rename = "b")]
    b: DateTime<Utc>,

    #[serde(rename = "c")]
    c: Option<String>,

    #[serde(rename = "d")]
    d: Option<D>,
}

