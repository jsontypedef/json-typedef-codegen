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
    static ref INITIALISMS: BTreeSet<String> = include_str!("initialisms")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case_with_initialisms(
                INITIALISMS.clone()
            ))
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::snake_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::screaming_snake_case())
        ));
}

pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("__init__.py".into()),
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
            strings_are_nullable: false,
            timestamps_are_nullable: false,
            arrays_are_nullable: false,
            dicts_are_nullable: false,
            aliases_are_nullable: false,
            enums_are_nullable: false,
            structs_are_nullable: false,
            discriminators_are_nullable: false,
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
        if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Any".into());

                "Any".into()
            }
            target::Expr::Boolean => "bool".into(),
            target::Expr::Int8 => "int".into(),
            target::Expr::Uint8 => "int".into(),
            target::Expr::Int16 => "int".into(),
            target::Expr::Uint16 => "int".into(),
            target::Expr::Int32 => "int".into(),
            target::Expr::Uint32 => "int".into(),
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "float".into(),
            target::Expr::String => "str".into(),
            target::Expr::Timestamp => "datetime".into(),
            target::Expr::ArrayOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("List".into());

                format!("List[{}]", sub_expr)
            }
            target::Expr::DictOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Dict".into());

                format!("Dict[str, {}]", sub_expr)
            }
            target::Expr::NullableOf(sub_expr) => {
                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .insert("Optional".into());

                format!("Optional[{}]", sub_expr)
            }
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

                state
                    .imports
                    .entry("datetime".into())
                    .or_default()
                    .extend(vec![
                        "datetime".into(),
                        "timedelta".into(),
                        "timezone".into(),
                    ]);

                writeln!(
                    out,
                    "# Code generated by jtd-codegen for Python v{}",
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(out)?;

                // To avoid having to "import match from re" (which we use in
                // _parse_rfc3339), we special-case this import.
                writeln!(out, "import re")?;
                for (module, idents) in &state.imports {
                    writeln!(
                        out,
                        "from {} import {}",
                        module,
                        idents.iter().cloned().collect::<Vec<_>>().join(", ")
                    )?;
                }

                writeln!(out)?;

                None
            }

            target::Item::Postamble => {
                writeln!(out)?;
                writeln!(out, "def _from_json_data(cls: Any, data: Any) -> Any:")?;
                writeln!(
                    out,
                    "    if data is None or cls in [bool, int, float, str, object] or cls is Any:"
                )?;
                writeln!(out, "        return data")?;
                writeln!(out, "    if cls is datetime:")?;
                writeln!(out, "        return _parse_rfc3339(data)")?;
                writeln!(out, "    if get_origin(cls) is Union:")?;
                writeln!(
                    out,
                    "        return _from_json_data(get_args(cls)[0], data)"
                )?;
                writeln!(out, "    if get_origin(cls) is list:")?;
                writeln!(
                    out,
                    "        return [_from_json_data(get_args(cls)[0], d) for d in data]"
                )?;
                writeln!(out, "    if get_origin(cls) is dict:")?;
                writeln!(out, "        return {{ k: _from_json_data(get_args(cls)[1], v) for k, v in data.items() }}")?;
                writeln!(out, "    return cls.from_json_data(data)")?;
                writeln!(out)?;
                writeln!(out, "def _to_json_data(data: Any) -> Any:")?;
                writeln!(
                    out,
                    "    if data is None or type(data) in [bool, int, float, str, object]:"
                )?;
                writeln!(out, "        return data")?;
                writeln!(out, "    if type(data) is datetime:")?;
                writeln!(out, "        return data.isoformat()")?;
                writeln!(out, "    if type(data) is list:")?;
                writeln!(out, "        return [_to_json_data(d) for d in data]")?;
                writeln!(out, "    if type(data) is dict:")?;
                writeln!(
                    out,
                    "        return {{ k: _to_json_data(v) for k, v in data.items() }}"
                )?;
                writeln!(out, "    return data.to_json_data()")?;
                writeln!(out)?;
                writeln!(out, "def _parse_rfc3339(s: str) -> datetime:")?;
                writeln!(out, "    datetime_re = r'^(\\d{{4}})-(\\d{{2}})-(\\d{{2}})[tT](\\d{{2}}):(\\d{{2}}):(\\d{{2}})(\\.\\d+)?([zZ]|((\\+|-)(\\d{{2}}):(\\d{{2}})))$'")?;
                writeln!(out, "    match = re.match(datetime_re, s)")?;
                writeln!(out, "    if not match:")?;
                writeln!(
                    out,
                    "        raise ValueError('Invalid RFC3339 date/time', s)"
                )?;
                writeln!(out)?;
                writeln!(
                    out,
                    "    (year, month, day, hour, minute, second, frac_seconds, offset,"
                )?;
                writeln!(out, "     *tz) = match.groups()")?;
                writeln!(out)?;
                writeln!(out, "    frac_seconds_parsed = None")?;
                writeln!(out, "    if frac_seconds:")?;
                writeln!(
                    out,
                    "        frac_seconds_parsed = int(float(frac_seconds) * 1_000_000)"
                )?;
                writeln!(out, "    else:")?;
                writeln!(out, "        frac_seconds_parsed = 0")?;
                writeln!(out)?;
                writeln!(out, "    tzinfo = None")?;
                writeln!(out, "    if offset == 'Z':")?;
                writeln!(out, "        tzinfo = timezone.utc")?;
                writeln!(out, "    else:")?;
                writeln!(out, "        hours = int(tz[2])")?;
                writeln!(out, "        minutes = int(tz[3])")?;
                writeln!(out, "        sign = 1 if tz[1] == '+' else -1")?;
                writeln!(out)?;
                writeln!(out, "        if minutes not in range(60):")?;
                writeln!(
                    out,
                    "            raise ValueError('minute offset must be in 0..59')"
                )?;
                writeln!(out)?;
                writeln!(
                    out,
                    "        tzinfo = timezone(timedelta(minutes=sign * (60 * hours + minutes)))"
                )?;
                writeln!(out)?;
                writeln!(out, "    second_parsed = int(second)")?;
                writeln!(out, "    if second_parsed == 60:")?;
                writeln!(out, "        second_parsed = 59")?;
                writeln!(out)?;
                writeln!(
                    out,
                    "    return datetime(int(year), int(month), int(day), int(hour), int(minute),"
                )?;
                writeln!(
                    out,
                    "                    second_parsed, frac_seconds_parsed, tzinfo)            "
                )?;

                None
            }

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                writeln!(out)?;
                writeln!(out, "@dataclass")?;
                writeln!(out, "class {}:", name)?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    value: '{}'", type_)?;
                writeln!(out)?;
                writeln!(out, "    @classmethod")?;
                writeln!(out, "    def from_json_data(cls, data: Any) -> '{}':", name)?;
                writeln!(out, "        return cls(_from_json_data({}, data))", type_)?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data(self) -> Any:")?;
                writeln!(out, "        return _to_json_data(self.value)")?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("enum".into())
                    .or_default()
                    .insert("Enum".into());

                writeln!(out)?;
                writeln!(out, "class {}(Enum):", name)?;
                write!(out, "{}", description(&metadata, 1))?;
                for member in &members {
                    writeln!(out, "    {} = {:?}", member.name, member.json_value,)?;
                    write!(
                        out,
                        "{}",
                        enum_variant_description(&metadata, 1, &member.json_value)
                    )?;
                }
                writeln!(out, "    @classmethod")?;
                writeln!(out, "    def from_json_data(cls, data: Any) -> '{}':", name)?;
                writeln!(out, "        return cls(data)")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data(self) -> Any:")?;
                writeln!(out, "        return self.value")?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .extend(vec!["Dict".into(), "Optional".into()]);

                writeln!(out)?;
                writeln!(out, "@dataclass")?;
                writeln!(out, "class {}:", name)?;
                write!(out, "{}", description(&metadata, 1))?;
                for field in &fields {
                    writeln!(out, "    {}: '{}'", field.name, field.type_,)?;
                    write!(out, "{}", description(&field.metadata, 1))?;
                }

                writeln!(out)?;
                writeln!(out, "    @classmethod")?;
                writeln!(out, "    def from_json_data(cls, data: Any) -> '{}':", name)?;
                writeln!(out, "        return cls(")?;
                for field in &fields {
                    writeln!(
                        out,
                        "            _from_json_data({}, data.get({:?})),",
                        field.type_, field.json_name
                    )?;
                }
                writeln!(out, "        )")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data(self) -> Any:")?;
                writeln!(out, "        data: Dict[str, Any] = {{}}")?;
                for field in &fields {
                    if field.optional {
                        writeln!(out, "        if self.{} is not None:", field.name)?;
                        writeln!(
                            out,
                            "             data[{:?}] = _to_json_data(self.{})",
                            field.json_name, field.name
                        )?;
                    } else {
                        writeln!(
                            out,
                            "        data[{:?}] = _to_json_data(self.{})",
                            field.json_name, field.name
                        )?;
                    }
                }
                writeln!(out, "        return data")?;

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                tag_field_name,
                tag_json_name,
                variants,
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                state
                    .imports
                    .entry("typing".into())
                    .or_default()
                    .extend(vec!["Type".into(), "Dict".into()]);

                writeln!(out)?;
                writeln!(out, "@dataclass")?;
                writeln!(out, "class {}:", name)?;
                write!(out, "{}", description(&metadata, 1))?;
                writeln!(out, "    {}: 'str'", tag_field_name)?;
                writeln!(out)?;
                writeln!(out, "    @classmethod")?;
                writeln!(out, "    def from_json_data(cls, data: Any) -> '{}':", name)?;
                writeln!(out, "        variants: Dict[str, Type[{}]] = {{", name)?;
                for variant in &variants {
                    writeln!(
                        out,
                        "            {:?}: {},",
                        variant.tag_value, variant.type_name
                    )?;
                }
                writeln!(out, "        }}")?;
                writeln!(out)?;
                writeln!(
                    out,
                    "        return variants[data[{:?}]].from_json_data(data)",
                    tag_json_name
                )?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data(self) -> Any:")?;
                writeln!(out, "        pass")?;
                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                parent_name,
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("pythonType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                state
                    .imports
                    .entry("dataclasses".into())
                    .or_default()
                    .insert("dataclass".into());

                writeln!(out)?;
                writeln!(out, "@dataclass")?;
                writeln!(out, "class {}({}):", name, parent_name)?;
                write!(out, "{}", description(&metadata, 1))?;
                for field in &fields {
                    writeln!(out, "    {}: '{}'", field.name, field.type_,)?;
                    write!(out, "{}", description(&field.metadata, 1))?;
                }

                writeln!(out)?;
                writeln!(out, "    @classmethod")?;
                writeln!(out, "    def from_json_data(cls, data: Any) -> '{}':", name)?;
                writeln!(out, "        return cls(")?;
                writeln!(out, "            {:?},", tag_value)?;
                for field in &fields {
                    writeln!(
                        out,
                        "            _from_json_data({}, data.get({:?})),",
                        field.type_, field.json_name
                    )?;
                }
                writeln!(out, "        )")?;
                writeln!(out)?;
                writeln!(out, "    def to_json_data(self) -> Any:")?;
                writeln!(
                    out,
                    "        data = {{ {:?}: {:?} }}",
                    tag_json_name, tag_value
                )?;
                for field in &fields {
                    if field.optional {
                        writeln!(out, "        if self.{} is not None:", field.name)?;
                        writeln!(
                            out,
                            "             data[{:?}] = _to_json_data(self.{})",
                            field.json_name, field.name
                        )?;
                    } else {
                        writeln!(
                            out,
                            "        data[{:?}] = _to_json_data(self.{})",
                            field.json_name, field.name
                        )?;
                    }
                }
                writeln!(out, "        return data")?;

                None
            }
        })
    }
}

#[derive(Default)]
pub struct FileState {
    imports: BTreeMap<String, BTreeSet<String>>,
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
    let out = jtd_codegen::target::fmt::comment_block(
        &format!("{}\"\"\"", prefix),
        &format!("{}", prefix),
        &format!("{}\"\"\"", prefix),
        s,
    );

    if out.is_empty() {
        out
    } else {
        out + "\n"
    }
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new());
    }

    mod optional_std_tests {
        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_properties
        );

        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_enum_values
        );
    }
}
