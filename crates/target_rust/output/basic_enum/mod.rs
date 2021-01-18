
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub enum Root {





	#[serde(rename = "Bar")]
	Bar,





	#[serde(rename = "Baz")]
	Baz,





	#[serde(rename = "Foo")]
	Foo,

}
