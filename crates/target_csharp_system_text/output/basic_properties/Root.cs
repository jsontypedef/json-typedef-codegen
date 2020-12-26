using System.Collections.Generic;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class Root
    {

        /// <summary>

        /// </summary>

        [JsonPropertyName("bar")]
        public string Bar { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("baz")]
        public IList<bool> Baz { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("foo")]
        public bool Foo { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("quux")]
        public IList<bool> Quux { get; set; }

    }
}
