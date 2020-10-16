use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum Discriminator {

    #[serde(rename = "v1")]
    V1(V1),

    #[serde(rename = "v2")]
    V2(V2),
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V1 {

    #[serde(rename = "user")]
    user: V1User,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V1User {

    #[serde(rename = "favoriteNumbers")]
    favorite_numbers: Vec<u32>,

    #[serde(rename = "id")]
    id: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V2 {

    #[serde(rename = "user")]
    user: V2User,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V2User {

    #[serde(rename = "favoriteNumbers")]
    favorite_numbers: Vec<String>,

    #[serde(rename = "id")]
    id: String,
}

