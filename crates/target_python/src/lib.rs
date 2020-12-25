use jtd_codegen::*;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

// todo: use keyword-avoiding inflectors
lazy_static! {
    // todo: more of an "item" naming convention, containing both types and
    // consts. What is the proper name for this, per the Go spec?
    static ref TYPE_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(CombiningInflector::new(Case::PascalCase));
    static ref FIELD_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::SnakeCase));
    static ref ENUM_VARIANT_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::ScreamingSnakeCase));
}

pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jtd_codegen::Target for Target {
    type FileState = FileState;

    fn file_partitioning() -> FilePartitioning {
        FilePartitioning::SingleFile("__init__.py".into())
    }

    fn enum_strategy() -> EnumStrategy {
        EnumStrategy::Modularized
    }

    fn name_type(name_parts: &[String]) -> String {
        TYPE_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_field(name_parts: &[String]) -> String {
        FIELD_NAMING_CONVENTION.inflect(name_parts)
    }

    fn name_enum_variant(name_parts: &[String]) -> String {
        ENUM_VARIANT_NAMING_CONVENTION.inflect(name_parts)
    }

    fn booleans_are_nullable() -> bool {
        false
    }

    fn strings_are_nullable() -> bool {
        false
    }

    fn timestamps_are_nullable() -> bool {
        false
    }

    fn arrays_are_nullable() -> bool {
        false
    }

    fn aliases_are_nullable() -> bool {
        false
    }

    fn enums_are_nullable() -> bool {
        false
    }

    fn structs_are_nullable() -> bool {
        false
    }

    fn discriminators_are_nullable() -> bool {
        false
    }

    fn boolean(&self, _state: &mut Self::FileState) -> String {
        format!("bool")
    }

    fn string(&self, _state: &mut Self::FileState) -> String {
        format!("str")
    }

    fn timestamp(&self, _state: &mut Self::FileState) -> String {
        format!("str")
    }

    fn nullable_of(&self, state: &mut Self::FileState, type_: String) -> String {
        state
            .imports
            .entry("typing".into())
            .or_default()
            .insert("Optional".into());
        format!("Optional[{}]", type_)
    }

    fn array_of(&self, state: &mut Self::FileState, type_: String) -> String {
        state
            .imports
            .entry("typing".into())
            .or_default()
            .insert("List".into());
        format!("List[{}]", type_)
    }

    fn write_preamble(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()> {
        state
            .imports
            .entry("typing".into())
            .or_default()
            .extend(vec![
                "Any".into(),
                "Union".into(),
                "get_origin".into(),
                "get_args".into(),
            ]);

        for (package, idents) in &state.imports {
            writeln!(
                out,
                "from {} import {}",
                package,
                idents.iter().cloned().collect::<Vec<_>>().join(", ")
            )?;
        }

        writeln!(out, "def _from_json(cls, data):")?;
        writeln!(
            out,
            "    if data is None or cls in [bool, int, float, str] or cls is Any:"
        )?;
        writeln!(out, "        return data")?;
        writeln!(out, "    if get_origin(cls) is Union:")?;
        writeln!(out, "        return _from_json(get_args(cls)[0], data)")?;
        writeln!(out, "    if get_origin(cls) is list:")?;
        writeln!(
            out,
            "        return [_from_json(get_args(cls)[0], d) for d in data]"
        )?;
        writeln!(out, "    if get_origin(cls) is dict:")?;
        writeln!(
            out,
            "        return {{ k: _from_json(get_args(cls)[1], v) for k, v in data.items() }}"
        )?;
        writeln!(out, "    return cls.from_json(data)")?;
        writeln!(out, "")?;
        writeln!(out, "def _to_json(data):")?;
        writeln!(
            out,
            "    if data is None or type(data) in [bool, int, float, str]:"
        )?;
        writeln!(out, "        return data")?;
        writeln!(out, "    if type(data) is list:")?;
        writeln!(out, "        return [_to_json(d) for d in data]")?;
        writeln!(out, "    if type(data) is dict:")?;
        writeln!(
            out,
            "        return {{ k: _to_json(v) for k, v in data.items() }}"
        )?;
        writeln!(out, "    return data.to_json()")?;

        Ok(())
    }

    fn write_alias(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String> {
        state
            .imports
            .entry("dataclasses".into())
            .or_default()
            .insert("dataclass".into());

        writeln!(out, "@dataclass")?;
        writeln!(out, "class {}:", alias.name)?;
        writeln!(out, "    value: {:?}", alias.type_)?;
        writeln!(out, "    @classmethod")?;
        writeln!(out, "    def from_json(cls, data) -> {:?}:", alias.name)?;
        writeln!(
            out,
            "        return {}(_from_json({}, data))",
            alias.name, alias.type_
        )?;
        writeln!(out, "    def to_json(self):")?;
        writeln!(out, "        return _to_json(self.value)")?;
        Ok(alias.name)
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String> {
        state
            .imports
            .entry("enum".into())
            .or_default()
            .insert("Enum".into());

        writeln!(out, "class {}(Enum):", enum_.name)?;
        for variant in enum_.variants {
            writeln!(out, "    {} = {:?}", variant.name, variant.json_value)?;
        }

        writeln!(out, "    @classmethod")?;
        writeln!(out, "    def from_json(cls, data) -> {:?}:", enum_.name)?;
        writeln!(out, "         return {}(data)", enum_.name)?;
        writeln!(out, "    def to_json(self):")?;
        writeln!(out, "        return self.value")?;

        Ok(enum_.name)
    }

    fn write_struct(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        struct_: Struct,
    ) -> Result<String> {
        state
            .imports
            .entry("dataclasses".into())
            .or_default()
            .insert("dataclass".into());

        writeln!(out, "@dataclass")?;
        writeln!(out, "class {}:", struct_.name)?;

        for field in &struct_.fields {
            writeln!(out, "    {}: {:?}", field.name, field.type_)?;
        }

        writeln!(out, "    @classmethod")?;
        writeln!(out, "    def from_json(cls, data) -> {:?}:", struct_.name)?;
        writeln!(out, "        return {}(", struct_.name)?;
        for field in &struct_.fields {
            writeln!(
                out,
                "            _from_json({}, data[{:?}]),",
                field.type_, field.json_name
            )?;
        }
        writeln!(out, "        )")?;

        writeln!(out, "    def to_json(self):")?;
        writeln!(out, "        return {{")?;
        for field in &struct_.fields {
            writeln!(
                out,
                "            {:?}: _to_json(self.{}),",
                field.json_name, field.name,
            )?;
        }
        writeln!(out, "        }}")?;

        Ok(struct_.name)
    }

    fn write_discriminator_variant(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        variant: DiscriminatorVariant,
    ) -> Result<String> {
        state
            .imports
            .entry("dataclasses".into())
            .or_default()
            .insert("dataclass".into());

        writeln!(out, "@dataclass")?;
        writeln!(out, "class {}({}):", variant.name, variant.parent_name)?;

        for field in &variant.fields {
            writeln!(out, "    {}: {:?}", field.name, field.type_)?;
        }

        writeln!(out, "    @classmethod")?;
        writeln!(out, "    def from_json(cls, data) -> {:?}:", variant.name)?;
        writeln!(out, "        return {}(", variant.name)?;
        writeln!(out, "            {:?},", variant.tag_json_value)?;
        for field in &variant.fields {
            writeln!(
                out,
                "            _from_json({}, data[{:?}]),",
                field.type_, field.json_name
            )?;
        }
        writeln!(out, "        )")?;

        writeln!(out, "    def to_json(self):")?;
        writeln!(out, "        return {{")?;
        writeln!(
            out,
            "            {:?}: {:?},",
            variant.tag_json_name, variant.tag_json_value
        )?;
        for field in &variant.fields {
            writeln!(
                out,
                "            {:?}: _to_json(self.{}),",
                field.json_name, field.name,
            )?;
        }
        writeln!(out, "        }}")?;

        Ok(variant.name)
    }

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator,
    ) -> Result<String> {
        state
            .imports
            .entry("dataclasses".into())
            .or_default()
            .insert("dataclass".into());

        writeln!(out, "@dataclass")?;
        writeln!(out, "class {}:", discriminator.name)?;
        writeln!(out, "    {}: str", discriminator.tag_json_name)?;

        writeln!(out, "    @classmethod")?;
        writeln!(
            out,
            "    def from_json(cls, data) -> {:?}:",
            discriminator.name
        )?;
        writeln!(out, "        return {{")?;
        for (tag_value, variant_name) in &discriminator.variants {
            writeln!(out, "            {:?}: {},", tag_value, variant_name)?;
        }
        writeln!(
            out,
            "        }}[data[{:?}]].from_json(data)",
            discriminator.tag_json_name
        )?;

        Ok(discriminator.name)
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeMap<String, BTreeSet<String>>,
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new());
    }
}
