using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootQuux : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "QUUX"; }

        [JsonPropertyName("quuz")]
        public string Quuz { get; set; }
    }
}
