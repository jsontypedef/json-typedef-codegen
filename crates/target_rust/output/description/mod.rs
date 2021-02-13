use serde::{Deserialize, Serialize};

/// A description for discriminator
#[derive(Serialize, Deserialize)]
#[serde(tag = "foo")]
pub enum RootDiscriminatorWithDescription {
    #[serde(rename = "bar")]
    Bar(RootDiscriminatorWithDescriptionBar),
}

/// A description for discriminator variant
#[derive(Serialize, Deserialize)]
pub struct RootDiscriminatorWithDescriptionBar {}

/// A description for enum
#[derive(Serialize, Deserialize)]
pub enum RootEnumWithDescription {
    /// A description for X
    #[serde(rename = "X")]
    X,

    /// A description for Y
    #[serde(rename = "Y")]
    Y,

    /// A description for Z
    #[serde(rename = "Z")]
    Z,
}

/// A description for properties
#[derive(Serialize, Deserialize)]
pub struct RootPropertiesWithDescription {}

#[derive(Serialize, Deserialize)]
pub struct Root {
    /// A description for discriminator
    #[serde(rename = "discriminator_with_description")]
    pub discriminatorWithDescription: RootDiscriminatorWithDescription,

    /// A description for enum
    #[serde(rename = "enum_with_description")]
    pub enumWithDescription: RootEnumWithDescription,

    /// Whereas disregard and contempt for human rights have resulted in
    /// barbarous acts which have outraged the conscience of mankind, and the
    /// advent of a world in which human beings shall enjoy freedom of speech
    /// and belief and freedom from fear and want has been proclaimed as the
    /// highest aspiration of the common people,
    #[serde(rename = "long_description")]
    pub longDescription: String,

    /// A description for properties
    #[serde(rename = "properties_with_description")]
    pub propertiesWithDescription: RootPropertiesWithDescription,

    /// A description for ref
    #[serde(rename = "ref_with_description")]
    pub refWithDescription: Baz,

    /// A description for string
    #[serde(rename = "string_with_description")]
    pub stringWithDescription: String,
}

/// A description for a definition
pub type Baz = String;
