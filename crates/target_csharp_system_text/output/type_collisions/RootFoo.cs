using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootFoo
    {
        [JsonPropertyName("bar")]
        public RootFooBar Bar { get; set; }
    }
}
