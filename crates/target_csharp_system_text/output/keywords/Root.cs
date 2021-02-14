using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("for")]
        public For For { get; set; }

        [JsonPropertyName("object")]
        public Object_ Object_ { get; set; }
    }
}
