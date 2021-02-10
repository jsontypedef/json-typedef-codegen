
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


    notnull_ref_notnull_string: 'NotnullRefNotnullString'
    """

    """


    notnull_ref_null_string: 'NotnullRefNullString'
    """

    """


    notnull_string: 'NotnullString'
    """

    """


    null_ref_notnull_string: 'NullRefNotnullString'
    """

    """


    null_ref_null_string: 'NullRefNullString'
    """

    """


    null_string: 'NullString'
    """

    """



    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(

            _from_json(NotnullRefNotnullString, data.get("notnull_ref_notnull_string")),

            _from_json(NotnullRefNullString, data.get("notnull_ref_null_string")),

            _from_json(NotnullString, data.get("notnull_string")),

            _from_json(NullRefNotnullString, data.get("null_ref_notnull_string")),

            _from_json(NullRefNullString, data.get("null_ref_null_string")),

            _from_json(NullString, data.get("null_string")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}

        
        out["notnull_ref_notnull_string"] = _to_json(self.notnull_ref_notnull_string)
        

        
        out["notnull_ref_null_string"] = _to_json(self.notnull_ref_null_string)
        

        
        out["notnull_string"] = _to_json(self.notnull_string)
        

        
        out["null_ref_notnull_string"] = _to_json(self.null_ref_notnull_string)
        

        
        out["null_ref_null_string"] = _to_json(self.null_ref_null_string)
        

        
        out["null_string"] = _to_json(self.null_string)
        

        return out
@dataclass
class NotnullRefNotnullString:
    """

    """

    value: "NotnullString"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NotnullRefNotnullString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(NotnullString, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class NotnullRefNullString:
    """

    """

    value: "NullString"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NotnullRefNullString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(NullString, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class NotnullString:
    """

    """

    value: "str"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NotnullString":
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
class NullRefNotnullString:
    """

    """

    value: "Optional[NotnullString]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NullRefNotnullString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(Optional[NotnullString], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class NullRefNullString:
    """

    """

    value: "Optional[NullString]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NullRefNullString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(Optional[NullString], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class NullString:
    """

    """

    value: "Optional[str]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "NullString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(Optional[str], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
