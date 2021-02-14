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

        [JsonPropertyName("coordinates")]
        public IList<Position> Coordinates { get; set; }

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox? Bbox { get; set; }
    }
}
