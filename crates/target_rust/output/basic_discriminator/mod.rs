
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
#[serde(tag = "foo")]
pub enum Root {

	#[serde(rename = "BAR_BAZ")]
	Barbaz(RootBarBaz),

	#[serde(rename = "QUUX")]
	Quux(RootQuux),

}




#[derive(Serialize, Deserialize)]
pub struct RootBarBaz {





    #[serde(rename = "baz")]
    
    pub baz: String,

}




#[derive(Serialize, Deserialize)]
pub struct RootQuux {





    #[serde(rename = "quuz")]
    
    pub quuz: String,

}
