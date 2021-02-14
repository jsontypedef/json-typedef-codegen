using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("foo")]
        public RootFoo Foo { get; set; }

        [JsonPropertyName("foo_bar")]
        public RootFooBar0 FooBar { get; set; }
    }
}
