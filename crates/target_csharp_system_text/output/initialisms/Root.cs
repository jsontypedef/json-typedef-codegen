using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("http")]
        public string Http { get; set; }

        [JsonPropertyName("id")]
        public string Id { get; set; }

        [JsonPropertyName("nested_id_initialism")]
        public RootNestedIdInitialism NestedIdInitialism { get; set; }

        [JsonPropertyName("utf8")]
        public string Utf8 { get; set; }

        [JsonPropertyName("word_with_embedded_id_initialism")]
        public string WordWithEmbeddedIdInitialism { get; set; }

        [JsonPropertyName("word_with_trailing_initialism_id")]
        public string WordWithTrailingInitialismId { get; set; }
    }
}
