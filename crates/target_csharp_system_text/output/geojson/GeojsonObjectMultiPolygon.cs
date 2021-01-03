
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// For type "MultiPolygon", the "coordinates" member is an array of
    /// Polygon coordinate arrays.
    /// </summary>

    public class GeojsonObjectMultiPolygon : GeojsonObject
    {
        [JsonPropertyName("type")]
        public string Type_ { get => "MultiPolygon"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("coordinates")]
        public IList<LinearRing> Coordinates { get; set; }

    }
}
