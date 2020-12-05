using System.Collections.Generic;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("bar")]
        public string Bar { get; set; }
        [JsonPropertyName("baz")]
        public IList<bool> Baz { get; set; }
        [JsonPropertyName("foo")]
        public bool Foo { get; set; }
        [JsonPropertyName("quux")]
        public IList<bool> Quux { get; set; }
    }
}
