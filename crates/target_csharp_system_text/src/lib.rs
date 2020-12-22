use jtd_codegen::*;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::io::Write;

// todo: use keyword-avoiding inflectors
lazy_static! {
    static ref TYPE_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(CombiningInflector::new(Case::PascalCase));
    static ref FIELD_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::PascalCase));
    static ref ENUM_VARIANT_NAMING_CONVENTION: Box<dyn Inflector + Send + Sync> =
        Box::new(TailInflector::new(Case::PascalCase));
}

pub struct Target {
    namespace: String,
}

impl Target {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }
}

impl jtd_codegen::Target for Target {
    type FileState = FileState;

    fn file_partitioning() -> FilePartitioning {
        FilePartitioning::FilePerType("cs".into())
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
        true
    }

    fn timestamps_are_nullable() -> bool {
        false
    }

    fn arrays_are_nullable() -> bool {
        true
    }

    fn aliases_are_nullable() -> bool {
        true
    }

    fn enums_are_nullable() -> bool {
        false
    }

    fn structs_are_nullable() -> bool {
        true
    }

    fn discriminators_are_nullable() -> bool {
        true
    }

    fn boolean(&self, _state: &mut Self::FileState) -> String {
        format!("bool")
    }

    fn string(&self, _state: &mut Self::FileState) -> String {
        format!("string")
    }

    fn timestamp(&self, state: &mut Self::FileState) -> String {
        state.imports.insert("System".into());
        format!("DateTimeOffset")
    }

    fn nullable_of(&self, _state: &mut Self::FileState, type_: String) -> String {
        format!("{}?", type_)
    }

    fn array_of(&self, state: &mut Self::FileState, type_: String) -> String {
        state.imports.insert("System.Collections.Generic".into());
        format!("IList<{}>", type_)
    }

    fn write_preamble(&self, state: &mut Self::FileState, out: &mut dyn Write) -> Result<()> {
        for namespace in &state.imports {
            writeln!(out, "using {};", namespace)?;
        }

        Ok(())
    }

    fn write_alias(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        alias: Alias,
    ) -> Result<String> {
        state.imports.insert("System".into());

        state.imports.insert("System.Text.Json".into());

        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(
            out,
            "    [JsonConverter(typeof({}.JsonConverter))]",
            alias.name
        )?;
        writeln!(out, "    public class {}", alias.name)?;
        writeln!(out, "    {{")?;
        writeln!(out, "        public {} Value {{ get; set; }}", alias.type_)?;
        writeln!(
            out,
            "        public class JsonConverter : JsonConverter<{}>",
            alias.name
        )?;
        writeln!(out, "        {{")?;
        writeln!(out, "            public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", alias.name)?;
        writeln!(out, "            {{")?;
        writeln!(out, "                return new {} {{ Value = JsonSerializer.Deserialize<{}>(ref reader, options) }};", alias.name, alias.type_)?;
        writeln!(out, "            }}")?;
        writeln!(out, "            public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", alias.name)?;
        writeln!(out, "            {{")?;
        writeln!(
            out,
            "                JsonSerializer.Serialize<{}>(writer, value.Value, options);",
            alias.type_
        )?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(alias.name)
    }

    fn write_enum(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        enum_: Enum,
    ) -> Result<String> {
        state.imports.insert("System".into());

        state.imports.insert("System.Text.Json".into());

        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(
            out,
            "    [JsonConverter(typeof({}JsonConverter))]",
            enum_.name
        )?;
        writeln!(out, "    public enum {}", enum_.name)?;
        writeln!(out, "    {{")?;

        for variant in &enum_.variants {
            writeln!(out, "        {},", variant.name)?;
        }

        writeln!(out, "    }}")?;

        writeln!(
            out,
            "    public class {}JsonConverter : JsonConverter<{}>",
            enum_.name, enum_.name
        )?;
        writeln!(out, "    {{")?;
        writeln!(out, "        public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", enum_.name)?;
        writeln!(out, "        {{")?;
        writeln!(
            out,
            "            string value = JsonSerializer.Deserialize<string>(ref reader, options);"
        )?;
        writeln!(out, "            switch (value)")?;
        writeln!(out, "            {{")?;

        for variant in &enum_.variants {
            writeln!(out, "                case {:?}:", variant.json_value)?;
            writeln!(
                out,
                "                    return {}.{};",
                enum_.name, variant.name
            )?;
        }

        writeln!(out, "                default:")?;
        writeln!(out, "                    throw new ArgumentException(String.Format(\"Bad {} value: {{0}}\", value));", enum_.name)?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "        public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", enum_.name)?;
        writeln!(out, "        {{")?;
        writeln!(out, "            switch (value)")?;
        writeln!(out, "            {{")?;
        for variant in enum_.variants {
            writeln!(out, "                case {}.{}:", enum_.name, variant.name)?;
            writeln!(
                out,
                "                    JsonSerializer.Serialize<string>(writer, {:?}, options);",
                variant.json_value
            )?;
            writeln!(out, "                    return;")?;
        }
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

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
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(out, "    public class {}", struct_.name)?;
        writeln!(out, "    {{")?;

        for field in struct_.fields {
            writeln!(out, "        [JsonPropertyName({:?})]", field.json_name)?;
            writeln!(
                out,
                "        public {} {} {{ get; set; }}",
                field.type_, field.name
            )?;
        }

        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

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
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(
            out,
            "    public class {} : {}",
            variant.name, variant.parent_name
        )?;
        writeln!(out, "    {{")?;
        writeln!(
            out,
            "        [JsonPropertyName({:?})]",
            variant.tag_json_name
        )?;
        writeln!(
            out,
            "        public string {} {{ get => {:?}; }}",
            variant.tag_name, variant.tag_json_value
        )?;

        for field in variant.fields {
            writeln!(out, "        [JsonPropertyName({:?})]", field.json_name)?;
            writeln!(
                out,
                "        public {} {} {{ get; set; }}",
                field.type_, field.name
            )?;
        }

        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(variant.name)
    }

    fn write_discriminator(
        &self,
        state: &mut Self::FileState,
        out: &mut dyn Write,
        discriminator: Discriminator,
    ) -> Result<String> {
        state.imports.insert("System".into());

        state.imports.insert("System.Text.Json".into());

        state
            .imports
            .insert("System.Text.Json.Serialization".into());

        writeln!(out, "namespace {}", self.namespace)?;
        writeln!(out, "{{")?;
        writeln!(
            out,
            "    [JsonConverter(typeof({}JsonConverter))]",
            discriminator.name
        )?;
        writeln!(out, "    public abstract class {}", discriminator.name)?;
        writeln!(out, "    {{")?;
        writeln!(out, "    }}")?;
        writeln!(
            out,
            "    public class {}JsonConverter : JsonConverter<{}>",
            discriminator.name, discriminator.name
        )?;
        writeln!(out, "    {{")?;
        writeln!(out, "        public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", discriminator.name)?;
        writeln!(out, "        {{")?;
        writeln!(out, "            var readerCopy = reader;")?;
        writeln!(out, "            var tagValue = JsonDocument.ParseValue(ref reader).RootElement.GetProperty({:?}).GetString();", discriminator.tag_json_name)?;
        writeln!(out, "            switch (tagValue)")?;
        writeln!(out, "            {{")?;
        for (tag_value, variant) in discriminator.variants {
            writeln!(out, "                case {:?}:", tag_value)?;
            writeln!(out, "                    return JsonSerializer.Deserialize<{}>(ref readerCopy, options);", variant)?;
        }
        writeln!(out, "                default:")?;
        writeln!(out, "                    throw new ArgumentException(String.Format(\"Bad {} value: {{0}}\", tagValue));", discriminator.tag_name)?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "        public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", discriminator.name)?;
        writeln!(out, "        {{")?;
        writeln!(
            out,
            "            JsonSerializer.Serialize(writer, value, value.GetType(), options);"
        )?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(discriminator.name)
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("JtdCodegenE2E".into()));
    }
}
