using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class RootFooBar
    {
        [JsonPropertyName("x")]
        public bool X { get; set; }
    }
}
