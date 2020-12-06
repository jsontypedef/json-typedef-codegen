using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("Foo")]
        public string Foo { get; set; }
        [JsonPropertyName("foo")]
        public string Foo0 { get; set; }
    }
}
