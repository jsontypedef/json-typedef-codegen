
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// A GeoJSON object with type "GeometryCollection" is a Geometry
    /// object. A GeometryCollection has a member with the name
    /// "geometries".  The value of "geometries" is an array.  Each element
    /// of this array is a GeoJSON Geometry object.  It is possible for this
    /// array to be empty.
    /// 
    /// Unlike the other geometry types described above, a
    /// GeometryCollection can be a heterogeneous composition of smaller
    /// Geometry objects.  For example, a Geometry object in the shape of a
    /// lowercase roman "i" can be composed of one point and one LineString.
    /// 
    /// GeometryCollections have a different syntax from single type
    /// Geometry objects (Point, LineString, and Polygon) and homogeneously
    /// typed multipart Geometry objects (MultiPoint, MultiLineString, and
    /// MultiPolygon) but have no different semantics.  Although a
    /// GeometryCollection object has no "coordinates" member, it does have
    /// coordinates: the coordinates of all its parts belong to the
    /// collection.  The "geometries" member of a GeometryCollection
    /// describes the parts of this composition.  Implementations SHOULD NOT
    /// apply any additional semantics to the "geometries" array.
    /// 
    /// To maximize interoperability, implementations SHOULD avoid nested
    /// GeometryCollections.  Furthermore, GeometryCollections composed of a
    /// single part or a number of parts of a single type SHOULD be avoided
    /// when that single part or a single object of multipart type
    /// (MultiPoint, MultiLineString, or MultiPolygon) could be used
    /// instead.
    /// </summary>

    public class GeojsonObjectGeometryCollection : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "GeometryCollection"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("geometries")]
        
        public IList<GeojsonObject> Geometries { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox Bbox { get; set; }

    }
}
