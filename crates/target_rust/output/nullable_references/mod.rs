use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "notnull_ref_notnull_string")]
    pub notnullRefNotnullString: NotnullRefNotnullString,

    #[serde(rename = "notnull_ref_null_string")]
    pub notnullRefNullString: NotnullRefNullString,

    #[serde(rename = "notnull_string")]
    pub notnullString: NotnullString,

    #[serde(rename = "null_ref_notnull_string")]
    pub nullRefNotnullString: NullRefNotnullString,

    #[serde(rename = "null_ref_null_string")]
    pub nullRefNullString: NullRefNullString,

    #[serde(rename = "null_string")]
    pub nullString: NullString,
}

pub type NotnullRefNotnullString = NotnullString;

pub type NotnullRefNullString = NullString;

pub type NotnullString = String;

pub type NullRefNotnullString = Option<Box<NotnullString>>;

pub type NullRefNullString = Option<Box<NullString>>;

pub type NullString = Option<Box<String>>;
