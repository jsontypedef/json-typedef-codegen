
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class RootBarBaz : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "BAR_BAZ"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("baz")]
        
        public string Baz { get; set; }

    }
}
