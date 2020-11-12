# Code generated by jtd-codegen {{ version }}. DO NOT EDIT.

{% for (module, identifiers) in imports -%}
from {{ module }} import {{ identifiers|join(", ") }}
{% endfor -%}
{# whitespace between imports and classes #}
{% let classes = classes.into_lookup_vec() %}
{% for (name, class) in classes %}
{%- match class -%}
{%- when Class::TypeWrapper with (type_wrapper) -%}
@dataclass
class {{ name }}:
{{- type_wrapper.description|comment_block(1) }}
    value: '{{ type_wrapper.type_.to_string(classes, definition_name_ids) }}'
    """
    The underlying value being wrapped. This value is populated by from_json,
    and is serialized by to_json.
    """

    @classmethod
    def from_json(cls, data) -> '{{ name}}':
        return cls(_from_json({{ type_wrapper.type_.to_string(classes, definition_name_ids)}}, data))

    def to_json(self):
        return _to_json(self.value)
{%- when Class::Enum with (enum_) -%}
class {{ name }}(Enum):
{{- enum_.description|comment_block(1) }}
{%- for (name, member) in enum_.members %}
    {{ name }} = {{ member.value }}
{{- member.description|comment_block(1) -}}
{% endfor %}

    @classmethod
    def from_json(cls, data) -> '{{ name }}':
        return cls(data)

    def to_json(self):
        return self.value
{%- when Class::Dataclass with (dataclass) -%}
@dataclass
class {{ name }}:
{{- dataclass.description|comment_block(1) }}
{%- for (name, field) in dataclass.fields %}
    {{ name }}: '{{ field.type_.to_string(classes, definition_name_ids) }}'
{{- field.description|comment_block(1) -}}
{% endfor %}

    @classmethod
    def from_json(cls, data) -> '{{ name }}':
        return cls(
{%- for (_, field) in dataclass.fields %}
            _from_json({{ field.type_.to_string(classes, definition_name_ids) }}, data[{{ field.json_name }}]),
{%- endfor %}
        )

    def to_json(self):
        return {
{%- for (name, field) in dataclass.fields %}
            {{ field.json_name}}: _to_json(self.{{name}}),
{%- endfor %}
        }
{%- when Class::Discriminator with (discriminator) -%}
@dataclass
class {{ name }}:
{{- discriminator.description|comment_block(1) }}
    {{ discriminator.discriminator_name }}: str
{%- for (name, variant) in discriminator.variants %}
    {{ name }}: 'Optional[{{ variant.type_.to_string(classes, definition_name_ids) }}]' = None
{%- endfor %}

    @classmethod
    def from_json(cls, data) -> '{{ name }}':
        result = cls(data[{{ discriminator.discriminator_json_name }}])
{%- for (name, variant) in discriminator.variants %}
        {% if loop.first %}if{% else %}elif{% endif %} data[{{ discriminator.discriminator_json_name }}] == {{ variant.discriminator_value }}:
            result.{{ name }} = {{ variant.type_.to_string(classes, definition_name_ids) }}.from_json(data)
{%- endfor %}
        return result

    def to_json(self):
        result = {}
{%- for (name, variant) in discriminator.variants %}
        {% if loop.first %}if{% else %}elif{% endif %} self.{{ discriminator.discriminator_name }} == {{ variant.discriminator_value }}:
            result = self.{{ name }}.to_json()
{%- endfor %}
        result[{{ discriminator.discriminator_json_name }}] = self.{{ discriminator.discriminator_name }}
        return result
{% endmatch %}
{% endfor %}
def _from_json(cls, data):
    if data is None or cls in [bool, int, float, str] or cls is Any:
        return data
    if get_origin(cls) is Union:
        return _from_json(get_args(cls)[0], data)
    if get_origin(cls) is list:
        return [_from_json(get_args(cls)[0], d) for d in data]
    if get_origin(cls) is dict:
        return { k: _from_json(get_args(cls)[1], v) for k, v in data.items() }
    return cls.from_json(data)

def _to_json(data):
    if data is None or type(data) in [bool, int, float, str]:
        return data
    if type(data) is list:
        return [_to_json(d) for d in data]
    if type(data) is dict:
        return { k: _to_json(v) for k, v in data.items() }
    return data.to_json()