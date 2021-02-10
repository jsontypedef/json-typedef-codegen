
from dataclasses import dataclass

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
class Root:
    """

    """


    for_: 'For'
    """

    """


    object: 'Object'
    """

    """



    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(For, data.get("for")),

            _from_json(Object, data.get("object")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["for"] = _to_json(self.for_)
        

        
        out["object"] = _to_json(self.object)
        

        return out
@dataclass
class For:
    """

    """

    value: "str"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "For":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(str, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class Object:
    """

    """

    value: "str"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "Object":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(str, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
