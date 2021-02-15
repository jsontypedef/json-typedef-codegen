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
class RootNestedIDInitialism:
    json: 'str'
    normalword: 'str'

    @classmethod
    def from_json(cls, data) -> 'RootNestedIDInitialism':
        return cls(
            _from_json(str, data.get("json")),
            _from_json(str, data.get("normalword")),
        )

    def to_json(self):
        data = {}
        data["json"] = _to_json(self.json)
        data["normalword"] = _to_json(self.normalword)
        return data

@dataclass
class Root:
    http: 'str'
    id: 'str'
    nested_id_initialism: 'RootNestedIDInitialism'
    utf8: 'str'
    word_with_embedded_id_initialism: 'str'
    word_with_trailing_initialism_id: 'str'

    @classmethod
    def from_json(cls, data) -> 'Root':
        return cls(
            _from_json(str, data.get("http")),
            _from_json(str, data.get("id")),
            _from_json(RootNestedIDInitialism, data.get("nested_id_initialism")),
            _from_json(str, data.get("utf8")),
            _from_json(str, data.get("word_with_embedded_id_initialism")),
            _from_json(str, data.get("word_with_trailing_initialism_id")),
        )

    def to_json(self):
        data = {}
        data["http"] = _to_json(self.http)
        data["id"] = _to_json(self.id)
        data["nested_id_initialism"] = _to_json(self.nested_id_initialism)
        data["utf8"] = _to_json(self.utf8)
        data["word_with_embedded_id_initialism"] = _to_json(self.word_with_embedded_id_initialism)
        data["word_with_trailing_initialism_id"] = _to_json(self.word_with_trailing_initialism_id)
        return data
