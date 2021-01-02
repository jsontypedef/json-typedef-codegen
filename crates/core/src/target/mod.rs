pub mod fmt;
pub mod inflect;
pub mod metadata;

use crate::error::Result;
use metadata::Metadata;
use std::io::Write;

pub trait Target {
    type FileState: Default;

    fn strategy(&self) -> Strategy;
    fn name(&self, kind: NameableKind, name_parts: &[String]) -> String;
    fn expr(&self, state: &mut Self::FileState, metadata: Metadata, expr: Expr) -> String;
    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut Self::FileState,
        item: Item,
    ) -> Result<Option<String>>;
}

pub struct Strategy {
    pub file_partitioning: FilePartitioningStrategy,
    pub enum_member_naming: EnumMemberNamingStrategy,
    pub booleans_are_nullable: bool,
    pub int8s_are_nullable: bool,
    pub uint8s_are_nullable: bool,
    pub int16s_are_nullable: bool,
    pub uint16s_are_nullable: bool,
    pub int32s_are_nullable: bool,
    pub uint32s_are_nullable: bool,
    pub float32s_are_nullable: bool,
    pub float64s_are_nullable: bool,
    pub strings_are_nullable: bool,
    pub timestamps_are_nullable: bool,
    pub arrays_are_nullable: bool,
    pub dicts_are_nullable: bool,
    pub aliases_are_nullable: bool,
    pub enums_are_nullable: bool,
    pub structs_are_nullable: bool,
    pub discriminators_are_nullable: bool,
}

pub enum FilePartitioningStrategy {
    FilePerType(String),
    SingleFile(String),
}

pub enum EnumMemberNamingStrategy {
    Modularized,
    Unmodularized,
}

pub enum NameableKind {
    Type,
    Field,
    EnumMember,
}

pub enum Expr {
    Empty,
    Boolean,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Float32,
    Float64,
    String,
    Timestamp,
    ArrayOf(String),
    DictOf(String),
    NullableOf(String),
}

pub enum Item {
    Preamble,
    Alias {
        metadata: Metadata,
        name: String,
        type_: String,
    },
    Enum {
        metadata: Metadata,
        name: String,
        members: Vec<EnumMember>,
    },
    Struct {
        metadata: Metadata,
        name: String,
        has_additional: bool,
        fields: Vec<Field>,
    },
    Discriminator {
        metadata: Metadata,
        name: String,
        tag_field_name: String,
        tag_json_name: String,
        variants: Vec<DiscriminatorVariantInfo>,
    },
    DiscriminatorVariant {
        metadata: Metadata,
        name: String,
        parent_name: String,
        tag_field_name: String,
        tag_json_name: String,
        tag_value: String,
        fields: Vec<Field>,
    },
}

pub struct EnumMember {
    pub name: String,
    pub json_value: String,
}

pub struct Field {
    pub metadata: Metadata,
    pub name: String,
    pub json_name: String,
    pub optional: bool,
    pub type_: String,
}

pub struct DiscriminatorVariantInfo {
    pub type_name: String,
    pub field_name: String,
    pub tag_value: String,
}
