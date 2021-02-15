package com.example;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

/**
 * A Geometry object represents points, curves, and surfaces in coordinate
 * space.  Every Geometry object is a GeoJSON object no matter where it
 * occurs in a GeoJSON text.
 * 
 * o  The value of a Geometry object's "type" member MUST be one of the
 *     seven geometry types (see Section 1.4).
 * 
 * o  A GeoJSON Geometry object of any type other than
 *     "GeometryCollection" has a member with the name "coordinates". The
 *     value of the "coordinates" member is an array.  The structure of the
 *     elements in this array is determined by the type of geometry.
 *     GeoJSON processors MAY interpret Geometry objects with empty
 *     "coordinates" arrays as null objects.
 */
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "type")
@JsonSubTypes({
    @JsonSubTypes.Type(name = "Feature", value = GeojsonObjectFeature.class),
    @JsonSubTypes.Type(name = "FeatureCollection", value = GeojsonObjectFeatureCollection.class),
    @JsonSubTypes.Type(name = "GeometryCollection", value = GeojsonObjectGeometryCollection.class),
    @JsonSubTypes.Type(name = "LineString", value = GeojsonObjectLineString.class),
    @JsonSubTypes.Type(name = "MultiLineString", value = GeojsonObjectMultiLineString.class),
    @JsonSubTypes.Type(name = "MultiPoint", value = GeojsonObjectMultiPoint.class),
    @JsonSubTypes.Type(name = "MultiPolygon", value = GeojsonObjectMultiPolygon.class),
    @JsonSubTypes.Type(name = "Point", value = GeojsonObjectPoint.class),
    @JsonSubTypes.Type(name = "Polygon", value = GeojsonObjectPolygon.class),
})
public abstract class GeojsonObject {
}
