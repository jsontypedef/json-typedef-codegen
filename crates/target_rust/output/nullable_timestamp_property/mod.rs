
use chrono::{DateTime, FixedOffset};

use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub struct Root {





    #[serde(rename = "foo")]
    
    pub foo: Option<Box<DateTime<FixedOffset>>>,

}
