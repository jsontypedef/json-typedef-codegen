from dataclasses import dataclass
from enum import Enum
from typing import Any, Union, get_args, get_origin
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
@dataclass
class Baz:
    value: "str"
    @classmethod
    def from_json(cls, data) -> "Baz":
        return Baz(_from_json(str, data))
    def to_json(self):
        return _to_json(self.value)
@dataclass
class RootDiscriminatorWithDescription:
    foo: str
    @classmethod
    def from_json(cls, data) -> "RootDiscriminatorWithDescription":
        return {
            "bar": RootDiscriminatorWithDescriptionBar,
        }[data["foo"]].from_json(data)
@dataclass
class RootDiscriminatorWithDescriptionBar(RootDiscriminatorWithDescription):
    @classmethod
    def from_json(cls, data) -> "RootDiscriminatorWithDescriptionBar":
        return RootDiscriminatorWithDescriptionBar(
            "bar",
        )
    def to_json(self):
        return {
            "foo": "bar",
        }
class RootEnumWithDescription(Enum):
    X = "X"
    Y = "Y"
    Z = "Z"
    @classmethod
    def from_json(cls, data) -> "RootEnumWithDescription":
         return RootEnumWithDescription(data)
    def to_json(self):
        return self.value
@dataclass
class RootPropertiesWithDescription:
    @classmethod
    def from_json(cls, data) -> "RootPropertiesWithDescription":
        return RootPropertiesWithDescription(
        )
    def to_json(self):
        return {
        }
@dataclass
class Root:
    discriminator_with_description: "RootDiscriminatorWithDescription"
    enum_with_description: "RootEnumWithDescription"
    long_description: "str"
    properties_with_description: "RootPropertiesWithDescription"
    ref_with_description: "Baz"
    string_with_description: "str"
    @classmethod
    def from_json(cls, data) -> "Root":
        return Root(
            _from_json(RootDiscriminatorWithDescription, data["discriminator_with_description"]),
            _from_json(RootEnumWithDescription, data["enum_with_description"]),
            _from_json(str, data["long_description"]),
            _from_json(RootPropertiesWithDescription, data["properties_with_description"]),
            _from_json(Baz, data["ref_with_description"]),
            _from_json(str, data["string_with_description"]),
        )
    def to_json(self):
        return {
            "discriminator_with_description": _to_json(self.discriminator_with_description),
            "enum_with_description": _to_json(self.enum_with_description),
            "long_description": _to_json(self.long_description),
            "properties_with_description": _to_json(self.properties_with_description),
            "ref_with_description": _to_json(self.ref_with_description),
            "string_with_description": _to_json(self.string_with_description),
        }
