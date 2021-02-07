
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub enum Root0 {





	#[serde(rename = "Bar")]
	Bar,





	#[serde(rename = "Baz")]
	Baz,





	#[serde(rename = "Foo")]
	Foo,

}




pub type Root = Option<Box<Root0>>;