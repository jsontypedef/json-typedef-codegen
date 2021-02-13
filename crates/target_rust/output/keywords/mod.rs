use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "for")]
    pub for_: For,

    #[serde(rename = "object")]
    pub object: Object,
}

pub type For = String;

pub type Object = String;
