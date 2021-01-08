
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

    Foo: str

    @classmethod
    def from_json(cls, data) -> "RootDiscriminatorWithDescription":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return {

            "bar": RootDiscriminatorWithDescriptionBar,

        }[data["foo"]].from_json(data)

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        pass # subclasses will implement this
@dataclass
class RootDiscriminatorWithDescriptionBar(RootDiscriminatorWithDescription):
    """
    A description for discriminator variant
    """



    @classmethod
    def from_json(cls, data) -> "RootDiscriminatorWithDescriptionBar":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "bar",

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["foo"] = "bar"

        return out
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
    def from_json(cls, data) -> "RootEnumWithDescription":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(data)

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return self.value
@dataclass
class RootPropertiesWithDescription:
    """
    A description for properties
    """



    @classmethod
    def from_json(cls, data) -> "RootPropertiesWithDescription":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        return out
@dataclass
class Root:
    """

    """


    DiscriminatorWithDescription: 'RootDiscriminatorWithDescription'
    """
    A description for discriminator
    """


    EnumWithDescription: 'RootEnumWithDescription'
    """
    A description for enum
    """


    LongDescription: 'str'
    """
    Whereas disregard and contempt for human rights have resulted in barbarous
    acts which have outraged the conscience of mankind, and the advent of a
    world in which human beings shall enjoy freedom of speech and belief and
    freedom from fear and want has been proclaimed as the highest aspiration of
    the common people,
    """


    PropertiesWithDescription: 'RootPropertiesWithDescription'
    """
    A description for properties
    """


    RefWithDescription: 'Baz'
    """
    A description for ref
    """


    StringWithDescription: 'str'
    """
    A description for string
    """



    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(RootDiscriminatorWithDescription, data.get("discriminator_with_description")),

            _from_json(RootEnumWithDescription, data.get("enum_with_description")),

            _from_json(str, data.get("long_description")),

            _from_json(RootPropertiesWithDescription, data.get("properties_with_description")),

            _from_json(Baz, data.get("ref_with_description")),

            _from_json(str, data.get("string_with_description")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["discriminator_with_description"] = _to_json(self.DiscriminatorWithDescription)
        

        
        out["enum_with_description"] = _to_json(self.EnumWithDescription)
        

        
        out["long_description"] = _to_json(self.LongDescription)
        

        
        out["properties_with_description"] = _to_json(self.PropertiesWithDescription)
        

        
        out["ref_with_description"] = _to_json(self.RefWithDescription)
        

        
        out["string_with_description"] = _to_json(self.StringWithDescription)
        

        return out
@dataclass
class Baz:
    """

    """

    value: "str"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "Baz":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(str, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
