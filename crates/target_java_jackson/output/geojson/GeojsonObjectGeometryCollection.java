package com.example;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.util.List;

/**
 * A GeoJSON object with type "GeometryCollection" is a Geometry
 * object. A GeometryCollection has a member with the name
 * "geometries".  The value of "geometries" is an array.  Each element
 * of this array is a GeoJSON Geometry object.  It is possible for this
 * array to be empty.
 * 
 * Unlike the other geometry types described above, a
 * GeometryCollection can be a heterogeneous composition of smaller
 * Geometry objects.  For example, a Geometry object in the shape of a
 * lowercase roman "i" can be composed of one point and one LineString.
 * 
 * GeometryCollections have a different syntax from single type
 * Geometry objects (Point, LineString, and Polygon) and homogeneously
 * typed multipart Geometry objects (MultiPoint, MultiLineString, and
 * MultiPolygon) but have no different semantics.  Although a
 * GeometryCollection object has no "coordinates" member, it does have
 * coordinates: the coordinates of all its parts belong to the
 * collection.  The "geometries" member of a GeometryCollection
 * describes the parts of this composition.  Implementations SHOULD NOT
 * apply any additional semantics to the "geometries" array.
 * 
 * To maximize interoperability, implementations SHOULD avoid nested
 * GeometryCollections.  Furthermore, GeometryCollections composed of a
 * single part or a number of parts of a single type SHOULD be avoided
 * when that single part or a single object of multipart type
 * (MultiPoint, MultiLineString, or MultiPolygon) could be used
 * instead.
 */
@JsonSerialize
@JsonIgnoreProperties(ignoreUnknown = true)
public class GeojsonObjectGeometryCollection extends GeojsonObject {
    @JsonProperty("geometries")
    private List<GeojsonObject> geometries;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bbox")
    private BoundingBox bbox;

    public GeojsonObjectGeometryCollection() {
    }

    /**
     * Getter for geometries.<p>
     */
    public List<GeojsonObject> getGeometries() {
        return geometries;
    }

    /**
     * Setter for geometries.<p>
     */
    public void setGeometries(List<GeojsonObject> geometries) {
        this.geometries = geometries;
    }

    /**
     * Getter for bbox.<p>
     */
    public BoundingBox getBbox() {
        return bbox;
    }

    /**
     * Setter for bbox.<p>
     */
    public void setBbox(BoundingBox bbox) {
        this.bbox = bbox;
    }
}
