using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootBar : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "bar"; }

        [JsonPropertyName("baz")]
        public string Baz { get; set; }
    }
}
