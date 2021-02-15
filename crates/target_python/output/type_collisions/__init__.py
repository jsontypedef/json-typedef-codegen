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
class RootFooBar:
    x: 'bool'

    @classmethod
    def from_json(cls, data) -> 'RootFooBar':
        return cls(
            _from_json(bool, data.get("x")),
        )

    def to_json(self):
        data = {}
        data["x"] = _to_json(self.x)
        return data

@dataclass
class RootFoo:
    bar: 'RootFooBar'

    @classmethod
    def from_json(cls, data) -> 'RootFoo':
        return cls(
            _from_json(RootFooBar, data.get("bar")),
        )

    def to_json(self):
        data = {}
        data["bar"] = _to_json(self.bar)
        return data

@dataclass
class RootFooBar0:
    x: 'str'

    @classmethod
    def from_json(cls, data) -> 'RootFooBar0':
        return cls(
            _from_json(str, data.get("x")),
        )

    def to_json(self):
        data = {}
        data["x"] = _to_json(self.x)
        return data

@dataclass
class Root:
    foo: 'RootFoo'
    foo_bar: 'RootFooBar0'

    @classmethod
    def from_json(cls, data) -> 'Root':
        return cls(
            _from_json(RootFoo, data.get("foo")),
            _from_json(RootFooBar0, data.get("foo_bar")),
        )

    def to_json(self):
        data = {}
        data["foo"] = _to_json(self.foo)
        data["foo_bar"] = _to_json(self.foo_bar)
        return data
