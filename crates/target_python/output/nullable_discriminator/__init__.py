
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
class Root0:
    """

    """

    Foo: str

    @classmethod
    def from_json(cls, data) -> "Root0":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return {

            "bar": RootBar,

            "quux": RootQuux,

        }[data["foo"]].from_json(data)

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        pass # subclasses will implement this
@dataclass
class RootBar(Root0):
    """

    """


    Baz: 'str'
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootBar":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "bar",

            _from_json(str, data.get("baz")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["foo"] = "bar"

        
        out["baz"] = _to_json(self.Baz)
        

        return out
@dataclass
class RootQuux(Root0):
    """

    """


    Quuz: 'str'
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootQuux":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "quux",

            _from_json(str, data.get("quuz")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["foo"] = "quux"

        
        out["quuz"] = _to_json(self.Quuz)
        

        return out
@dataclass
class Root:
    """

    """

    value: "Optional[Root0]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(Optional[Root0], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
