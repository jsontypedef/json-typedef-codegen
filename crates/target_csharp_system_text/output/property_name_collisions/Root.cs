using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class Root
    {

        /// <summary>

        /// </summary>

        [JsonPropertyName("Foo")]
        public string Foo { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("foo")]
        public string Foo0 { get; set; }

    }
}
