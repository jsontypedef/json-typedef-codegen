pub mod fmt;
pub mod inflect;
pub mod metadata;

use crate::error::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Write;

pub trait Target {
    type FileState: Default;

    fn file_partitioning() -> FilePartitioning;
    fn enum_strategy() -> EnumStrategy;

    fn booleans_are_nullable() -> bool;
    fn strings_are_nullable() -> bool;
    fn timestamps_are_nullable() -> bool;
    fn arrays_are_nullable() -> bool;
    fn aliases_are_nullable() -> bool;
    fn enums_are_nullable() -> bool;
    fn structs_are_nullable() -> bool;
    fn discriminators_are_nullable() -> bool;

    fn name_type(name_parts: &[String]) -> String;
    fn name_field(name_parts: &[String]) -> String;
    fn name_enum_variant(name_parts: &[String]) -> String;

    fn boolean(&self, state: &mut Self::FileState) -> String;
    fn string(&self, state: &mut Self::FileState) -> String;
    fn timestamp(&self, state: &mut Self::FileState) -> String;

    fn nullable_of(&self, state: &mut Self::FileState, type_: String) -> String;

    fn array_of(&self, state: &mut Self::FileState, type_: String) -> String;

    fn write_preamble<'a>(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()>;

    fn write_alias<'a>(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String>;

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String>;

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct,
    ) -> Result<String>;

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        variant: DiscriminatorVariant,
    ) -> Result<String>;

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator,
    ) -> Result<String>;
}

pub enum FilePartitioning {
    SingleFile(String),
    FilePerType(String),
}

pub enum EnumStrategy {
    Modularized,
    Unmodularized,
}

pub struct Alias {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub type_: String,
}

pub struct Enum {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone)]
pub struct EnumVariant {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub json_value: String,
}

pub struct Struct {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub has_additional: bool,
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub name: String,
    pub json_name: String,
    pub metadata: BTreeMap<String, Value>,
    pub optional: bool,
    pub type_: String,
}

pub struct DiscriminatorVariant {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub parent_name: String,
    pub tag_name: String,
    pub tag_json_name: String,
    pub tag_json_value: String,
    pub fields: Vec<StructField>,
}

pub struct Discriminator {
    pub name: String,
    pub metadata: BTreeMap<String, Value>,
    pub tag_name: String,
    pub tag_json_name: String,
    pub variants: BTreeMap<String, String>,
}
