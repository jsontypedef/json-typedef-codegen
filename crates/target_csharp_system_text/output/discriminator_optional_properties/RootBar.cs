
using System.Collections.Generic;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class RootBar : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "bar"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("baz")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public IList<string> Baz { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("quux")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public bool? Quux { get; set; }

    }
}
