use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case())
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
}

pub struct Target {
    namespace: String,
}

impl Target {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::FilePerType("cs".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::WrapWithNullable,
            booleans_are_nullable: false,
            int8s_are_nullable: false,
            uint8s_are_nullable: false,
            int16s_are_nullable: false,
            uint16s_are_nullable: false,
            int32s_are_nullable: false,
            uint32s_are_nullable: false,
            float32s_are_nullable: false,
            float64s_are_nullable: false,
            strings_are_nullable: true,
            timestamps_are_nullable: false,
            arrays_are_nullable: true,
            dicts_are_nullable: true,
            aliases_are_nullable: true,
            enums_are_nullable: false,
            structs_are_nullable: true,
            discriminators_are_nullable: true,
        }
    }

    fn name(&self, kind: target::NameableKind, parts: &[String]) -> String {
        match kind {
            target::NameableKind::Type => TYPE_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::Field => FIELD_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::EnumMember => ENUM_MEMBER_NAMING_CONVENTION.inflect(parts),
        }
    }

    fn expr(
        &self,
        state: &mut FileState,
        metadata: metadata::Metadata,
        expr: target::Expr,
    ) -> String {
        if let Some(s) = metadata
            .get("csharpSystemTextType")
            .and_then(|v| v.as_str())
        {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "object".into(),
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "sbyte".into(),
            target::Expr::Uint8 => "byte".into(),
            target::Expr::Int16 => "short".into(),
            target::Expr::Uint16 => "ushort".into(),
            target::Expr::Int32 => "int".into(),
            target::Expr::Uint32 => "uint".into(),
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "double".into(),
            target::Expr::String => "string".into(),
            target::Expr::Timestamp => {
                state.imports.insert("System".into());
                "DateTimeOffset".into()
            }
            target::Expr::ArrayOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("csharpSystemTextContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<{}>", s, sub_expr);
                }

                state.imports.insert("System.Collections.Generic".into());
                format!("IList<{}>", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                if let Some(s) = metadata
                    .get("csharpSystemTextContainer")
                    .and_then(|v| v.as_str())
                {
                    return format!("{}<string, {}>", s, sub_expr);
                }

                state.imports.insert("System.Collections.Generic".into());
                format!("IDictionary<string, {}>", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => format!("{}?", sub_expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut FileState,
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Auxiliary { .. } => {
                // No auxiliary files needed.
                None
            }

            target::Item::Preamble => {
                writeln!(
                    out,
                    "// Code generated by jtd-codegen for C# + System.Text.Json v{}",
                    env!("CARGO_PKG_VERSION")
                )?;

                if !state.imports.is_empty() {
                    writeln!(out)?;
                }

                for namespace in &state.imports {
                    writeln!(out, "using {};", namespace)?;
                }

                writeln!(out)?;

                None
            }

            target::Item::Postamble => None,

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(out, "namespace {}", &self.namespace)?;
                writeln!(out, "{{")?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    [JsonConverter(typeof({}JsonConverter))]", name)?;
                writeln!(out, "    public class {}", name)?;
                writeln!(out, "    {{")?;
                writeln!(out, "        /// <summary>")?;
                writeln!(out, "        /// The underlying data being wrapped.")?;
                writeln!(out, "        /// </summary>")?;
                writeln!(out, "        public {} Value {{ get; set; }}", type_)?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
                writeln!(
                    out,
                    "    public class {0}JsonConverter : JsonConverter<{0}>",
                    name
                )?;
                writeln!(out, "    {{")?;
                writeln!(out, "        public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(out, "            return new {} {{ Value = JsonSerializer.Deserialize<{}>(ref reader, options) }};", name, type_)?;
                writeln!(out, "        }}")?;
                writeln!(out)?;
                writeln!(out, "        public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(
                    out,
                    "            JsonSerializer.Serialize<{}>(writer, value.Value, options);",
                    type_
                )?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(out, "namespace {}", &self.namespace)?;
                writeln!(out, "{{")?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    [JsonConverter(typeof({}JsonConverter))]", name)?;
                writeln!(out, "    public enum {}", name)?;
                writeln!(out, "    {{")?;
                for (index, member) in members.iter().enumerate() {
                    if index != 0 {
                        writeln!(out)?;
                    }

                    write!(
                        out,
                        "{}",
                        enum_variant_description(&metadata, 2, &member.json_value)
                    )?;
                    writeln!(out, "        {},", member.name)?;
                }
                writeln!(out, "    }}")?;

                writeln!(
                    out,
                    "    public class {0}JsonConverter : JsonConverter<{0}>",
                    name
                )?;
                writeln!(out, "    {{")?;
                writeln!(out, "        public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(out, "            string value = JsonSerializer.Deserialize<string>(ref reader, options);")?;
                writeln!(out, "            switch (value)")?;
                writeln!(out, "            {{")?;
                for member in &members {
                    writeln!(out, "                case {:?}:", member.json_value)?;
                    writeln!(out, "                    return {}.{};", name, member.name)?;
                }
                writeln!(out, "                default:")?;
                writeln!(out, "                    throw new ArgumentException(String.Format(\"Bad {} value: {{0}}\", value));", name)?;
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out)?;
                writeln!(out, "        public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(out, "            switch (value)")?;
                writeln!(out, "            {{")?;
                for member in &members {
                    writeln!(out, "                case {}.{}:", name, member.name)?;
                    writeln!(out, "                    JsonSerializer.Serialize<string>(writer, {:?}, options);", member.json_value)?;
                    writeln!(out, "                    return;")?;
                }
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .insert("System.Text.Json.Serialization".into());

                writeln!(out, "namespace {}", &self.namespace)?;
                writeln!(out, "{{")?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    public class {}", name)?;
                writeln!(out, "    {{")?;
                for (index, field) in fields.into_iter().enumerate() {
                    if index != 0 {
                        writeln!(out)?;
                    }

                    write!(out, "{}", description(&field.metadata, 2))?;
                    writeln!(out, "        [JsonPropertyName({:?})]", field.json_name)?;
                    if field.optional {
                        writeln!(out, "        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]")?;
                    }
                    writeln!(
                        out,
                        "        public {} {} {{ get; set; }}",
                        field.type_, field.name
                    )?;
                }
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                tag_field_name,
                tag_json_name,
                variants,
            } => {
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state.imports.extend(vec![
                    "System".to_string(),
                    "System.Text.Json".to_string(),
                    "System.Text.Json.Serialization".to_string(),
                ]);

                writeln!(out, "namespace {}", &self.namespace)?;
                writeln!(out, "{{")?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    [JsonConverter(typeof({}JsonConverter))]", name)?;
                writeln!(out, "    public abstract class {}", name)?;
                writeln!(out, "    {{")?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
                writeln!(
                    out,
                    "    public class {0}JsonConverter : JsonConverter<{0}>",
                    name
                )?;
                writeln!(out, "    {{")?;
                writeln!(out, "        public override {} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(out, "            var readerCopy = reader;")?;
                writeln!(out, "            var tagValue = JsonDocument.ParseValue(ref reader).RootElement.GetProperty({:?}).GetString();", tag_json_name)?;
                writeln!(out)?;
                writeln!(out, "            switch (tagValue)")?;
                writeln!(out, "            {{")?;
                for variant in &variants {
                    writeln!(out, "                case {:?}:", variant.tag_value)?;
                    writeln!(out, "                    return JsonSerializer.Deserialize<{}>(ref readerCopy, options);", variant.type_name)?;
                }
                writeln!(out, "                default:")?;
                writeln!(out, "                    throw new ArgumentException(String.Format(\"Bad {} value: {{0}}\", tagValue));", tag_field_name)?;
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out)?;
                writeln!(out, "        public override void Write(Utf8JsonWriter writer, {} value, JsonSerializerOptions options)", name)?;
                writeln!(out, "        {{")?;
                writeln!(out, "            JsonSerializer.Serialize(writer, value, value.GetType(), options);")?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                parent_name,
                tag_field_name,
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata
                    .get("csharpSystemTextType")
                    .and_then(|v| v.as_str())
                {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .insert("System.Text.Json.Serialization".into());

                writeln!(out, "namespace {}", &self.namespace)?;
                writeln!(out, "{{")?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    public class {} : {}", name, parent_name)?;
                writeln!(out, "    {{")?;
                writeln!(out, "        [JsonPropertyName({:?})]", tag_json_name)?;
                writeln!(
                    out,
                    "        public string {} {{ get => {:?}; }}",
                    tag_field_name, tag_value
                )?;
                for field in fields {
                    writeln!(out)?;
                    write!(out, "{}", description(&field.metadata, 2))?;
                    writeln!(out, "        [JsonPropertyName({:?})]", field.json_name)?;
                    if field.optional {
                        writeln!(out, "        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]")?;
                    }
                    writeln!(
                        out,
                        "        public {} {} {{ get; set; }}",
                        field.type_, field.name
                    )?;
                }
                writeln!(out, "    }}")?;
                writeln!(out, "}}")?;

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeSet<String>,
}

fn description(metadata: &BTreeMap<String, Value>, indent: usize) -> String {
    doc(indent, jtd_codegen::target::metadata::description(metadata))
}

fn enum_variant_description(
    metadata: &BTreeMap<String, Value>,
    indent: usize,
    value: &str,
) -> String {
    doc(
        indent,
        jtd_codegen::target::metadata::enum_variant_description(metadata, value),
    )
}

fn doc(ident: usize, s: &str) -> String {
    let prefix = "    ".repeat(ident);
    jtd_codegen::target::fmt::comment_block(
        &format!("{}/// <summary>", prefix),
        &format!("{}/// ", prefix),
        &format!("{}/// </summary>", prefix),
        s,
    )
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new("JtdCodegenE2E".into()));
    }
}
