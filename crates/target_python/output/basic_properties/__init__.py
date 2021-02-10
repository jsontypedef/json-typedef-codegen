
from dataclasses import dataclass

from typing import Any, List, Optional, Union, get_args, get_origin

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


    bar: 'str'
    """

    """


    baz: 'List[bool]'
    """

    """


    foo: 'bool'
    """

    """


    quux: 'List[bool]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(str, data.get("bar")),

            _from_json(List[bool], data.get("baz")),

            _from_json(bool, data.get("foo")),

            _from_json(List[bool], data.get("quux")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["bar"] = _to_json(self.bar)
        

        
        out["baz"] = _to_json(self.baz)
        

        
        out["foo"] = _to_json(self.foo)
        

        
        out["quux"] = _to_json(self.quux)
        

        return out
