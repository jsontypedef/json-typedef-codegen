using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "Point", the "coordinates" member is a single position.
    /// </summary>
    public class GeojsonObjectPoint : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "Point"; }

        [JsonPropertyName("coordinates")]
        public Position Coordinates { get; set; }

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox? Bbox { get; set; }
    }
}
