// Code generated by jtd-codegen for C# + System.Text.Json v0.2.0

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
