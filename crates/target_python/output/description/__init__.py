from dataclasses import dataclass
from enum import Enum
from typing import Any, Optional, Union, get_args, get_origin

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
class RootDiscriminatorWithDescription:
    """
    A description for discriminator
    """

    foo: 'str'

    @classmethod
    def from_json(cls, data) -> 'RootDiscriminatorWithDescription':
        return {
            "bar": RootDiscriminatorWithDescriptionBar,
        }[data["foo"]].from_json(data)

    def to_json(self):
        pass

@dataclass
class RootDiscriminatorWithDescriptionBar(RootDiscriminatorWithDescription):
    """
    A description for discriminator variant
    """


    @classmethod
    def from_json(cls, data) -> 'RootDiscriminatorWithDescriptionBar':
        return cls(
            "bar",
        )

    def to_json(self):
        data = { "foo": "bar" }
        return data

class RootEnumWithDescription(Enum):
    """
    A description for enum
    """

    X = "X"
    """
    A description for X
    """

    Y = "Y"
    """
    A description for Y
    """

    Z = "Z"
    """
    A description for Z
    """

    @classmethod
    def from_json(cls, data) -> 'RootEnumWithDescription':
        return cls(data)

    def to_json(self):
        return self.value

@dataclass
class RootPropertiesWithDescription:
    """
    A description for properties
    """


    @classmethod
    def from_json(cls, data) -> 'RootPropertiesWithDescription':
        return cls(
        )

    def to_json(self):
        data = {}
        return data

@dataclass
class Root:
    discriminator_with_description: 'RootDiscriminatorWithDescription'
    """
    A description for discriminator
    """

    enum_with_description: 'RootEnumWithDescription'
    """
    A description for enum
    """

    long_description: 'str'
    """
    Whereas disregard and contempt for human rights have resulted in barbarous
    acts which have outraged the conscience of mankind, and the advent of a
    world in which human beings shall enjoy freedom of speech and belief and
    freedom from fear and want has been proclaimed as the highest aspiration of
    the common people,
    """

    properties_with_description: 'RootPropertiesWithDescription'
    """
    A description for properties
    """

    ref_with_description: 'Baz'
    """
    A description for ref
    """

    string_with_description: 'str'
    """
    A description for string
    """


    @classmethod
    def from_json(cls, data) -> 'Root':
        return cls(
            _from_json(RootDiscriminatorWithDescription, data.get("discriminator_with_description")),
            _from_json(RootEnumWithDescription, data.get("enum_with_description")),
            _from_json(str, data.get("long_description")),
            _from_json(RootPropertiesWithDescription, data.get("properties_with_description")),
            _from_json(Baz, data.get("ref_with_description")),
            _from_json(str, data.get("string_with_description")),
        )

    def to_json(self):
        data = {}
        data["discriminator_with_description"] = _to_json(self.discriminator_with_description)
        data["enum_with_description"] = _to_json(self.enum_with_description)
        data["long_description"] = _to_json(self.long_description)
        data["properties_with_description"] = _to_json(self.properties_with_description)
        data["ref_with_description"] = _to_json(self.ref_with_description)
        data["string_with_description"] = _to_json(self.string_with_description)
        return data

@dataclass
class Baz:
    """
    A description for a definition
    """

    value: 'str'

    @classmethod
    def from_json(cls, data) -> 'Baz':
        return cls(_from_json(str, data))

    def to_json(self):
        return _to_json(self.value)
