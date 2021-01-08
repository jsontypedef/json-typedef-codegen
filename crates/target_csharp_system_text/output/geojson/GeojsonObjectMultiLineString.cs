
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "MultiLineString", the "coordinates" member is an array of
    /// LineString coordinate arrays.
    /// </summary>

    public class GeojsonObjectMultiLineString : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "MultiLineString"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        
        public IList<Position> Coordinates { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox? Bbox { get; set; }

    }
}
