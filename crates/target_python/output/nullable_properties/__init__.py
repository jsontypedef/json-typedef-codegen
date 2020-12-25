from dataclasses import dataclass
from typing import Any, List, Optional, Union, get_args, get_origin
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
class Root0:
    bar: "str"
    baz: "List[bool]"
    foo: "bool"
    quux: "List[bool]"
    @classmethod
    def from_json(cls, data) -> "Root0":
        return Root0(
            _from_json(str, data["bar"]),
            _from_json(List[bool], data["baz"]),
            _from_json(bool, data["foo"]),
            _from_json(List[bool], data["quux"]),
        )
    def to_json(self):
        return {
            "bar": _to_json(self.bar),
            "baz": _to_json(self.baz),
            "foo": _to_json(self.foo),
            "quux": _to_json(self.quux),
        }
@dataclass
class Root:
    value: "Optional[Root0]"
    @classmethod
    def from_json(cls, data) -> "Root":
        return Root(_from_json(Optional[Root0], data))
    def to_json(self):
        return _to_json(self.value)
