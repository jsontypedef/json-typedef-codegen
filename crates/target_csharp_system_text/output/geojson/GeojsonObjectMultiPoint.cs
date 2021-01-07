
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

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        
        public IList<Position> Coordinates { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("bbox")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public BoundingBox Bbox { get; set; }

    }
}
