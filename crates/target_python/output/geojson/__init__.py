
from dataclasses import dataclass

from typing import Any, Dict, List, Optional, Union, get_args, get_origin

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

    value: "GeojsonObject"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "Root":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(GeojsonObject, data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class BoundingBox:
    """
    A GeoJSON object MAY have a member named "bbox" to include information
    on the coordinate range for its Geometries, Features, or
    FeatureCollections.  The value of the bbox member MUST be an array of
    length 2*n where n is the number of dimensions represented in the
    contained geometries, with all axes of the most southwesterly point
    followed by all axes of the more northeasterly point.  The axes order of
    a bbox follows the axes order of geometries.
    
    The "bbox" values define shapes with edges that follow lines of constant
    longitude, latitude, and elevation.
    
    The four lines of the bounding box are defined fully within the
    coordinate reference system; that is, for a box bounded by the values
    "west", "south", "east", and "north", every point on the northernmost
    line can be expressed as
    
    (lon, lat) = (west + (east - west) * t, north)
    
    with 0 <= t <= 1.
    
    Consider a set of point Features within the Fiji archipelago, straddling
    the antimeridian between 16 degrees S and 20 degrees S. The southwest
    corner of the box containing these Features is at 20 degrees S and 177
    degrees E, and the northwest corner is at 16 degrees S and 178 degrees
    W.  The antimeridian-spanning GeoJSON bounding box for this
    FeatureCollection is
    
    "bbox": [177.0, -20.0, -178.0, -16.0]
    
    and covers 5 degrees of longitude.
    
    The complementary bounding box for the same latitude band, not crossing
    the antimeridian, is
    
    "bbox": [-178.0, -20.0, 177.0, -16.0]
    
    and covers 355 degrees of longitude.
    
    The latitude of the northeast corner is always greater than the latitude
    of the southwest corner, but bounding boxes that cross the antimeridian
    have a northeast corner longitude that is less than the longitude of the
    southwest corner.
    
    A bounding box that contains the North Pole extends from a southwest
    corner of "minlat" degrees N, 180 degrees W to a northeast corner of 90
    degrees N, 180 degrees E.  Viewed on a globe, this bounding box
    approximates a spherical cap bounded by the "minlat" circle of latitude.
    
    "bbox": [-180.0, minlat, 180.0, 90.0]
    
    A bounding box that contains the South Pole extends from a southwest
    corner of 90 degrees S, 180 degrees W to a northeast corner of "maxlat"
    degrees S, 180 degrees E.
    
    "bbox": [-180.0, -90.0, 180.0, maxlat]
    
    A bounding box that just touches the North Pole and forms a slice of an
    approximate spherical cap when viewed on a globe extends from a
    southwest corner of "minlat" degrees N and "westlon" degrees E to a
    northeast corner of 90 degrees N and "eastlon" degrees E.
    
    "bbox": [westlon, minlat, eastlon, 90.0]
    
    Similarly, a bounding box that just touches the South Pole and forms a
    slice of an approximate spherical cap when viewed on a globe has the
    following representation in GeoJSON.
    
    "bbox": [westlon, -90.0, eastlon, maxlat]
    
    Implementers MUST NOT use latitude values greater than 90 or less than
    -90 to imply an extent that is not a spherical cap.
    """

    value: "List[float]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "BoundingBox":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(List[float], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class GeojsonObject:
    """
    A Geometry object represents points, curves, and surfaces in coordinate
    space.  Every Geometry object is a GeoJSON object no matter where it
    occurs in a GeoJSON text.
    
    o  The value of a Geometry object's "type" member MUST be one of the
        seven geometry types (see Section 1.4).
    
    o  A GeoJSON Geometry object of any type other than
        "GeometryCollection" has a member with the name "coordinates". The
        value of the "coordinates" member is an array.  The structure of the
        elements in this array is determined by the type of geometry.
        GeoJSON processors MAY interpret Geometry objects with empty
        "coordinates" arrays as null objects.
    """

    type: str

    @classmethod
    def from_json(cls, data) -> "GeojsonObject":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return {

            "Feature": GeojsonObjectFeature,

            "FeatureCollection": GeojsonObjectFeatureCollection,

            "GeometryCollection": GeojsonObjectGeometryCollection,

            "LineString": GeojsonObjectLineString,

            "MultiLineString": GeojsonObjectMultiLineString,

            "MultiPoint": GeojsonObjectMultiPoint,

            "MultiPolygon": GeojsonObjectMultiPolygon,

            "Point": GeojsonObjectPoint,

            "Polygon": GeojsonObjectPolygon,

        }[data["type"]].from_json(data)

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        pass # subclasses will implement this
@dataclass
class GeojsonObjectFeature(GeojsonObject):
    """
    A Feature object represents a spatially bounded thing.  Every
    Feature object is a GeoJSON object no matter where it occurs in a
    GeoJSON text.
    
    o  A Feature object has a "type" member with the value "Feature".
    
    o  A Feature object has a member with the name "geometry".  The
        value of the geometry member SHALL be either a Geometry object
        as defined above or, in the case that the Feature is unlocated,
        a JSON null value.
    
    o  A Feature object has a member with the name "properties".  The
        value of the properties member is an object (any JSON object or
        a JSON null value).
    """


    geometry: 'Optional[GeojsonObject]'
    """
    The GeoJSON specification requires that these elements be
    GeoJSON geometry objects, but such a constraint can't be
    expressed in JSON Type Definition.
    
    It is semantically invalid at the GeoJSON level for this
    member to be any GeoJSON object type other than one of the
    geometry types.
    """


    properties: 'Dict[str, Any]'
    """

    """


    id: 'Any'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectFeature":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "Feature",

            _from_json(Optional[GeojsonObject], data.get("geometry")),

            _from_json(Dict[str, Any], data.get("properties")),

            _from_json(Any, data.get("id")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "Feature"

        
        out["geometry"] = _to_json(self.geometry)
        

        
        out["properties"] = _to_json(self.properties)
        

        
        if self.id is not None:
            out["id"] = _to_json(self.id)
        

        return out
@dataclass
class GeojsonObjectFeatureCollection(GeojsonObject):
    """
    A GeoJSON object with the type "FeatureCollection" is a
    FeatureCollection object.  A FeatureCollection object has a member
    with the name "features".  The value of "features" is a JSON array.
    Each element of the array is a Feature object as defined above.  It
    is possible for this array to be empty.
    """


    features: 'List[GeojsonObject]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectFeatureCollection":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "FeatureCollection",

            _from_json(List[GeojsonObject], data.get("features")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "FeatureCollection"

        
        out["features"] = _to_json(self.features)
        

        return out
@dataclass
class GeojsonObjectGeometryCollection(GeojsonObject):
    """
    A GeoJSON object with type "GeometryCollection" is a Geometry
    object. A GeometryCollection has a member with the name
    "geometries".  The value of "geometries" is an array.  Each element
    of this array is a GeoJSON Geometry object.  It is possible for this
    array to be empty.
    
    Unlike the other geometry types described above, a
    GeometryCollection can be a heterogeneous composition of smaller
    Geometry objects.  For example, a Geometry object in the shape of a
    lowercase roman "i" can be composed of one point and one LineString.
    
    GeometryCollections have a different syntax from single type
    Geometry objects (Point, LineString, and Polygon) and homogeneously
    typed multipart Geometry objects (MultiPoint, MultiLineString, and
    MultiPolygon) but have no different semantics.  Although a
    GeometryCollection object has no "coordinates" member, it does have
    coordinates: the coordinates of all its parts belong to the
    collection.  The "geometries" member of a GeometryCollection
    describes the parts of this composition.  Implementations SHOULD NOT
    apply any additional semantics to the "geometries" array.
    
    To maximize interoperability, implementations SHOULD avoid nested
    GeometryCollections.  Furthermore, GeometryCollections composed of a
    single part or a number of parts of a single type SHOULD be avoided
    when that single part or a single object of multipart type
    (MultiPoint, MultiLineString, or MultiPolygon) could be used
    instead.
    """


    geometries: 'List[GeojsonObject]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectGeometryCollection":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "GeometryCollection",

            _from_json(List[GeojsonObject], data.get("geometries")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "GeometryCollection"

        
        out["geometries"] = _to_json(self.geometries)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectLineString(GeojsonObject):
    """
    For type "LineString", the "coordinates" member is an array of two
    or more positions.
    """


    coordinates: 'List[Position]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectLineString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "LineString",

            _from_json(List[Position], data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "LineString"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectMultiLineString(GeojsonObject):
    """
    For type "MultiLineString", the "coordinates" member is an array of
    LineString coordinate arrays.
    """


    coordinates: 'List[Position]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectMultiLineString":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "MultiLineString",

            _from_json(List[Position], data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "MultiLineString"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectMultiPoint(GeojsonObject):
    """
    For type "MultiPoint", the "coordinates" member is an array of
    positions.
    """


    coordinates: 'List[Position]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectMultiPoint":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "MultiPoint",

            _from_json(List[Position], data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "MultiPoint"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectMultiPolygon(GeojsonObject):
    """
    For type "MultiPolygon", the "coordinates" member is an array of
    Polygon coordinate arrays.
    """


    coordinates: 'List[LinearRing]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectMultiPolygon":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "MultiPolygon",

            _from_json(List[LinearRing], data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "MultiPolygon"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectPoint(GeojsonObject):
    """
    For type "Point", the "coordinates" member is a single position.
    """


    coordinates: 'Position'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectPoint":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "Point",

            _from_json(Position, data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "Point"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class GeojsonObjectPolygon(GeojsonObject):
    """
    For type "Polygon", the "coordinates" member MUST be an array of
    linear ring coordinate arrays.
    """


    coordinates: 'List[LinearRing]'
    """

    """


    bbox: 'Optional[BoundingBox]'
    """

    """



    @classmethod
    def from_json(cls, data) -> "GeojsonObjectPolygon":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(
            "Polygon",

            _from_json(List[LinearRing], data.get("coordinates")),

            _from_json(Optional[BoundingBox], data.get("bbox")),

        )

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        out = {}
        out["type"] = "Polygon"

        
        out["coordinates"] = _to_json(self.coordinates)
        

        
        if self.bbox is not None:
            out["bbox"] = _to_json(self.bbox)
        

        return out
@dataclass
class LinearRing:
    """
    To specify a constraint specific to Polygons, it is useful to
    introduce the concept of a linear ring:
    
    o  A linear ring is a closed LineString with four or more positions.
    
    o  The first and last positions are equivalent, and they MUST contain
        identical values; their representation SHOULD also be identical.
    
    o  A linear ring is the boundary of a surface or the boundary of a
        hole in a surface.
    
    o  A linear ring MUST follow the right-hand rule with respect to the
        area it bounds, i.e., exterior rings are counterclockwise, and holes
        are clockwise.
    
    Note: the [GJ2008] specification did not discuss linear ring winding
    order.  For backwards compatibility, parsers SHOULD NOT reject Polygons
    that do not follow the right-hand rule.
    
    Though a linear ring is not explicitly represented as a GeoJSON geometry
    type, it leads to a canonical formulation of the Polygon geometry type
    definition as follows:
    
    For Polygons with more than one of these rings, the first MUST be the
    exterior ring, and any others MUST be interior rings.  The exterior ring
    bounds the surface, and the interior rings (if present) bound holes
    within the surface.
    """

    value: "List[Position]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "LinearRing":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(List[Position], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
@dataclass
class Position:
    """
    A position is the fundamental geometry construct.
    
    A position is an array of numbers.  There MUST be two or more elements.
    The first two elements are longitude and latitude, or easting and
    northing, precisely in that order and using decimal numbers.  Altitude
    or elevation MAY be included as an optional third element.
    
    Implementations SHOULD NOT extend positions beyond three elements
    because the semantics of extra elements are unspecified and ambiguous.
    Historically, some implementations have used a fourth element to carry a
    linear referencing measure (sometimes denoted as "M") or a numerical
    timestamp, but in most situations a parser will not be able to properly
    interpret these values.  The interpretation and meaning of additional
    elements is beyond the scope of this specification, and additional
    elements MAY be ignored by parsers.
    
    A line between two positions is a straight Cartesian line, the shortest
    line between those two points in the coordinate reference system (see
    Section 4).
    
    In other words, every point on a line that does not cross the
    antimeridian between a point (lon0, lat0) and (lon1, lat1) can be
    calculated as
    
    F(lon, lat) = (lon0 + (lon1 - lon0) * t, lat0 + (lat1 - lat0) * t)
    
    with t being a real number greater than or equal to 0 and smaller than
    or equal to 1.  Note that this line may markedly differ from the
    geodesic path along the curved surface of the reference ellipsoid.
    
    The same applies to the optional height element with the proviso that
    the direction of the height is as specified in the coordinate reference
    system.
    
    Note that, again, this does not mean that a surface with equal height
    follows, for example, the curvature of a body of water.  Nor is a
    surface of equal height perpendicular to a plumb line.
    """

    value: "List[float]"
    """
    The value being wrapped.
    """

    @classmethod
    def from_json(cls, data) -> "Position":
        """
        Construct an instance of this class from parsed JSON data.
        """

        return cls(_from_json(List[float], data))

    def to_json(self):
        """
        Generate JSON-ready data from an instance of this class.
        """

        return _to_json(self.value)
