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
    notnull_ref_notnull_string: 'NotnullRefNotnullString'
    notnull_ref_null_string: 'NotnullRefNullString'
    notnull_string: 'NotnullString'
    null_ref_notnull_string: 'NullRefNotnullString'
    null_ref_null_string: 'NullRefNullString'
    null_string: 'NullString'

    @classmethod
    def from_json(cls, data) -> 'Root':
        return cls(
            _from_json(NotnullRefNotnullString, data.get("notnull_ref_notnull_string")),
            _from_json(NotnullRefNullString, data.get("notnull_ref_null_string")),
            _from_json(NotnullString, data.get("notnull_string")),
            _from_json(NullRefNotnullString, data.get("null_ref_notnull_string")),
            _from_json(NullRefNullString, data.get("null_ref_null_string")),
            _from_json(NullString, data.get("null_string")),
        )

    def to_json(self):
        data = {}
        data["notnull_ref_notnull_string"] = _to_json(self.notnull_ref_notnull_string)
        data["notnull_ref_null_string"] = _to_json(self.notnull_ref_null_string)
        data["notnull_string"] = _to_json(self.notnull_string)
        data["null_ref_notnull_string"] = _to_json(self.null_ref_notnull_string)
        data["null_ref_null_string"] = _to_json(self.null_ref_null_string)
        data["null_string"] = _to_json(self.null_string)
        return data

@dataclass
class NotnullRefNotnullString:
    value: 'NotnullString'

    @classmethod
    def from_json(cls, data) -> 'NotnullRefNotnullString':
        return cls(_from_json(NotnullString, data))

    def to_json(self):
        return _to_json(self.value)

@dataclass
class NotnullRefNullString:
    value: 'NullString'

    @classmethod
    def from_json(cls, data) -> 'NotnullRefNullString':
        return cls(_from_json(NullString, data))

    def to_json(self):
        return _to_json(self.value)

@dataclass
class NotnullString:
    value: 'str'

    @classmethod
    def from_json(cls, data) -> 'NotnullString':
        return cls(_from_json(str, data))

    def to_json(self):
        return _to_json(self.value)

@dataclass
class NullRefNotnullString:
    value: 'Optional[NotnullString]'

    @classmethod
    def from_json(cls, data) -> 'NullRefNotnullString':
        return cls(_from_json(Optional[NotnullString], data))

    def to_json(self):
        return _to_json(self.value)

@dataclass
class NullRefNullString:
    value: 'Optional[NullString]'

    @classmethod
    def from_json(cls, data) -> 'NullRefNullString':
        return cls(_from_json(Optional[NullString], data))

    def to_json(self):
        return _to_json(self.value)

@dataclass
class NullString:
    value: 'Optional[str]'

    @classmethod
    def from_json(cls, data) -> 'NullString':
        return cls(_from_json(Optional[str], data))

    def to_json(self):
        return _to_json(self.value)
