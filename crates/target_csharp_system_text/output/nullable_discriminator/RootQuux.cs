using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class RootQuux : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "quux"; }
        [JsonPropertyName("quuz")]
        public string Quuz { get; set; }
    }
}
