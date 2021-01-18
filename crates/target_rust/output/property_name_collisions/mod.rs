
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub struct Root {





    #[serde(rename = "Foo")]
    
    pub foo: String,





    #[serde(rename = "foo")]
    
    pub foo0: String,

}
