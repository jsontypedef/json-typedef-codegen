
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// A Feature object represents a spatially bounded thing.  Every
    /// Feature object is a GeoJSON object no matter where it occurs in a
    /// GeoJSON text.
    /// 
    /// o  A Feature object has a "type" member with the value "Feature".
    /// 
    /// o  A Feature object has a member with the name "geometry".  The
    ///     value of the geometry member SHALL be either a Geometry object
    ///     as defined above or, in the case that the Feature is unlocated,
    ///     a JSON null value.
    /// 
    /// o  A Feature object has a member with the name "properties".  The
    ///     value of the properties member is an object (any JSON object or
    ///     a JSON null value).
    /// </summary>

    public class GeojsonObjectFeature : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "Feature"; }

        /// <summary>
        /// The GeoJSON specification requires that these elements be
        /// GeoJSON geometry objects, but such a constraint can't be
        /// expressed in JSON Type Definition.
        /// 
        /// It is semantically invalid at the GeoJSON level for this
        /// member to be any GeoJSON object type other than one of the
        /// geometry types.
        /// </summary>

        [JsonPropertyName("geometry")]
        public GeojsonObject? Geometry { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("properties")]
        public IDictionary<string, object> Properties { get; set; }

    }
}
