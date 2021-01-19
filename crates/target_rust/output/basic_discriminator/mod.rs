
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
#[serde(tag = "foo")]
pub enum Root {

	#[serde(rename = "bar")]
	Bar(RootBar),

	#[serde(rename = "quux")]
	Quux(RootQuux),

}




#[derive(Serialize, Deserialize)]
pub struct RootBar {





    #[serde(rename = "baz")]
    
    pub baz: String,

}




#[derive(Serialize, Deserialize)]
pub struct RootQuux {





    #[serde(rename = "quuz")]
    
    pub quuz: String,

}
