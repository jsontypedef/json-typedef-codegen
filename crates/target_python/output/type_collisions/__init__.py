from dataclasses import dataclass
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
class RootFooBar:
    x: "bool"
    @classmethod
    def from_json(cls, data) -> "RootFooBar":
        return RootFooBar(
            _from_json(bool, data["x"]),
        )
    def to_json(self):
        return {
            "x": _to_json(self.x),
        }
@dataclass
class RootFoo:
    bar: "RootFooBar"
    @classmethod
    def from_json(cls, data) -> "RootFoo":
        return RootFoo(
            _from_json(RootFooBar, data["bar"]),
        )
    def to_json(self):
        return {
            "bar": _to_json(self.bar),
        }
@dataclass
class RootFooBar0:
    x: "str"
    @classmethod
    def from_json(cls, data) -> "RootFooBar0":
        return RootFooBar0(
            _from_json(str, data["x"]),
        )
    def to_json(self):
        return {
            "x": _to_json(self.x),
        }
@dataclass
class Root:
    foo: "RootFoo"
    foo_bar: "RootFooBar0"
    @classmethod
    def from_json(cls, data) -> "Root":
        return Root(
            _from_json(RootFoo, data["foo"]),
            _from_json(RootFooBar0, data["foo_bar"]),
        )
    def to_json(self):
        return {
            "foo": _to_json(self.foo),
            "foo_bar": _to_json(self.foo_bar),
        }
