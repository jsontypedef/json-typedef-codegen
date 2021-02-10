
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
class RootFooBar(Enum):
    """

    """


    X = "x"
    """

    """


    Y = "y"
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootFooBar":
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
class RootFoo:
    """

    """


    bar: 'RootFooBar'
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootFoo":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(RootFooBar, data.get("bar")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["bar"] = _to_json(self.bar)
        

        return out
class RootFooBar0(Enum):
    """

    """


    X = "x"
    """

    """


    Y = "y"
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootFooBar0":
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
class Root:
    """

    """


    foo: 'RootFoo'
    """

    """


    foo_bar: 'RootFooBar0'
    """

    """



    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(RootFoo, data.get("foo")),

            _from_json(RootFooBar0, data.get("foo_bar")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["foo"] = _to_json(self.foo)
        

        
        out["foo_bar"] = _to_json(self.foo_bar)
        

        return out
