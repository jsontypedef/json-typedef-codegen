using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class RootQuux : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "quux"; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("quuz")]
        public string Quuz { get; set; }

    }
}
