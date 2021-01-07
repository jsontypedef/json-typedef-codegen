
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "Polygon", the "coordinates" member MUST be an array of
    /// linear ring coordinate arrays.
    /// </summary>

    public class GeojsonObjectPolygon : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "Polygon"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        
        public IList<LinearRing> Coordinates { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox Bbox { get; set; }

    }
}
