
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub struct Root0 {





    #[serde(rename = "bar")]
    
    pub bar: String,





    #[serde(rename = "baz")]
    
    pub baz: Vec<bool>,





    #[serde(rename = "foo")]
    
    pub foo: bool,





    #[serde(rename = "quux")]
    
    pub quux: Vec<bool>,

}




pub type Root = Option<Box<Root0>>;
