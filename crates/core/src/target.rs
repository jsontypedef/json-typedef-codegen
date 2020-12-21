use crate::{Inflector, Result};
use std::collections::BTreeMap;
use std::io::Write;

pub trait Target {
    type FileState: Default;
    type ExprMeta: ExprMeta;

    fn file_partitioning(&self) -> FilePartitioning;
    fn enum_strategy(&self) -> EnumStrategy;

    fn booleans_are_nullable(&self) -> bool;
    fn strings_are_nullable(&self) -> bool;
    fn timestamps_are_nullable(&self) -> bool;
    fn arrays_are_nullable(&self) -> bool;
    fn aliases_are_nullable(&self) -> bool;
    fn enums_are_nullable(&self) -> bool;
    fn structs_are_nullable(&self) -> bool;
    fn discriminators_are_nullable(&self) -> bool;

    fn name_type(&self, name_parts: &[String]) -> String;
    fn name_field(&self, name_parts: &[String]) -> String;
    fn name_enum_variant(&self, name_parts: &[String]) -> String;

    fn boolean(&self, state: &mut Self::FileState) -> Expr<Self::ExprMeta>;
    fn string(&self, state: &mut Self::FileState) -> Expr<Self::ExprMeta>;
    fn timestamp(&self, state: &mut Self::FileState) -> Expr<Self::ExprMeta>;

    fn nullable_of(
        &self,
        state: &mut Self::FileState,
        expr: Expr<Self::ExprMeta>,
    ) -> Expr<Self::ExprMeta>;

    fn array_of(
        &self,
        state: &mut Self::FileState,
        expr: Expr<Self::ExprMeta>,
    ) -> Expr<Self::ExprMeta>;

    fn write_preamble<'a>(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()>;

    fn write_alias<'a>(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias<Self::ExprMeta>,
    ) -> Result<Expr<Self::ExprMeta>>;

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<Expr<Self::ExprMeta>>;

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct<Self::ExprMeta>,
    ) -> Result<Expr<Self::ExprMeta>>;

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: DiscriminatorVariant<Self::ExprMeta>,
    ) -> Result<Expr<Self::ExprMeta>>;

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Discriminator<Self::ExprMeta>,
    ) -> Result<Expr<Self::ExprMeta>>;
}

#[derive(Clone)]
pub struct Expr<T> {
    pub expr: String,
    pub meta: T,
}

pub trait ExprMeta: PartialEq + Clone {
    fn universally_usable() -> Self;
}

pub enum FilePartitioning {
    SingleFile(String),
    FilePerType(String),
}

pub enum EnumStrategy {
    Modularized,
    Unmodularized,
}

pub struct Alias<T> {
    pub name: String,
    pub description: String,
    pub type_: Expr<T>,
}

pub struct Enum {
    pub name: String,
    pub description: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone)]
pub struct EnumVariant {
    pub name: String,
    pub description: String,
    pub json_value: String,
}

pub struct Struct<T> {
    pub name: String,
    pub description: String,
    pub has_additional: bool,
    pub fields: Vec<StructField<T>>,
}

pub struct StructField<T> {
    pub name: String,
    pub json_name: String,
    pub description: String,
    pub optional: bool,
    pub type_: Expr<T>,
}

pub struct DiscriminatorVariant<T> {
    pub name: String,
    pub description: String,
    pub parent_name: String,
    pub tag_name: String,
    pub tag_json_name: String,
    pub tag_json_value: String,
    pub fields: Vec<StructField<T>>,
}

pub struct Discriminator<T> {
    pub name: String,
    pub description: String,
    pub tag_name: String,
    pub tag_json_name: String,
    pub variants: BTreeMap<String, Expr<T>>,
}
