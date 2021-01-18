
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize)]
pub enum Root {





	#[serde(rename = "FOO")]
	Foo,





	#[serde(rename = "Foo")]
	Foo0,





	#[serde(rename = "foo")]
	Foo1,

}
