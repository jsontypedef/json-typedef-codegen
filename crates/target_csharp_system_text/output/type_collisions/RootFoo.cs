using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>
    /// </summary>

    public class RootFoo
    {

        /// <summary>
        /// </summary>

        [JsonPropertyName("bar")]
        public RootFooBar Bar { get; set; }

    }
}
