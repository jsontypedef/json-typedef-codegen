
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

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        
        public Position Coordinates { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox Bbox { get; set; }

    }
}
