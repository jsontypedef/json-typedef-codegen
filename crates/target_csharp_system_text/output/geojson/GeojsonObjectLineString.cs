
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "LineString", the "coordinates" member is an array of two
    /// or more positions.
    /// </summary>

    public class GeojsonObjectLineString : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "LineString"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        public IList<Position> Coordinates { get; set; }

    }
}
