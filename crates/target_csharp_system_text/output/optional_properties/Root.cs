
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
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public IList<string> Bar { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("baz")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public bool Baz { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("foo")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public string Foo { get; set; }

    }
}
