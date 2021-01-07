
from dataclasses import dataclass

from typing import Any, Optional, Union, get_args, get_origin

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
class RootFooBar:
    """

    """


    X: 'bool'
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootFooBar":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(bool, data.get("x")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["x"] = _to_json(self.X)
        

        return out
@dataclass
class RootFoo:
    """

    """


    Bar: 'RootFooBar'
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

        
        out["bar"] = _to_json(self.Bar)
        

        return out
@dataclass
class RootFooBar0:
    """

    """


    X: 'str'
    """

    """



    @classmethod
    def from_json(cls, data) -> "RootFooBar0":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(str, data.get("x")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["x"] = _to_json(self.X)
        

        return out
@dataclass
class Root:
    """

    """


    Foo: 'RootFoo'
    """

    """


    FooBar: 'RootFooBar0'
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

        
        out["foo"] = _to_json(self.Foo)
        

        
        out["foo_bar"] = _to_json(self.FooBar)
        

        return out
