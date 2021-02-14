using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "MultiPoint", the "coordinates" member is an array of
    /// positions.
    /// </summary>
    public class GeojsonObjectMultiPoint : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "MultiPoint"; }

        [JsonPropertyName("coordinates")]
        public IList<Position> Coordinates { get; set; }

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox? Bbox { get; set; }
    }
}
