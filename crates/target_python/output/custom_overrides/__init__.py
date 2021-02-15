from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Union, get_args, get_origin

def _from_json(cls, data):
    if data is None or cls in [bool, int, float, str, object] or cls is Any:
        return data
    if get_origin(cls) is Union:
        return _from_json(get_args(cls)[0], data)
    if get_origin(cls) is list:
        return [_from_json(get_args(cls)[0], d) for d in data]
    if get_origin(cls) is dict:
        return { k: _from_json(get_args(cls)[1], v) for k, v in data.items() }
    return cls.from_json(data)

def _to_json(data):
    if data is None or type(data) in [bool, int, float, str, object]:
        return data
    if type(data) is list:
        return [_to_json(d) for d in data]
    if type(data) is dict:
        return { k: _to_json(v) for k, v in data.items() }
    return data.to_json()

@dataclass
class RootOverrideTypeDiscriminatorBaz(object):

    @classmethod
    def from_json(cls, data) -> 'RootOverrideTypeDiscriminatorBaz':
        return cls(
            "baz",
        )

    def to_json(self):
        data = { "foo": "baz" }
        return data

@dataclass
class Root:
    override_elements_container: 'List[str]'
    override_type_discriminator: 'object'
    override_type_enum: 'object'
    override_type_expr: 'object'
    override_type_properties: 'object'
    override_values_container: 'Dict[str, str]'

    @classmethod
    def from_json(cls, data) -> 'Root':
        return cls(
            _from_json(List[str], data.get("override_elements_container")),
            _from_json(object, data.get("override_type_discriminator")),
            _from_json(object, data.get("override_type_enum")),
            _from_json(object, data.get("override_type_expr")),
            _from_json(object, data.get("override_type_properties")),
            _from_json(Dict[str, str], data.get("override_values_container")),
        )

    def to_json(self):
        data = {}
        data["override_elements_container"] = _to_json(self.override_elements_container)
        data["override_type_discriminator"] = _to_json(self.override_type_discriminator)
        data["override_type_enum"] = _to_json(self.override_type_enum)
        data["override_type_expr"] = _to_json(self.override_type_expr)
        data["override_type_properties"] = _to_json(self.override_type_properties)
        data["override_values_container"] = _to_json(self.override_values_container)
        return data
