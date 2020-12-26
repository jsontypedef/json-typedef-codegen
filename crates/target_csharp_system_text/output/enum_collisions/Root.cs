using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>
    /// </summary>

    public class Root
    {

        /// <summary>
        /// </summary>

        [JsonPropertyName("foo")]
        public RootFoo Foo { get; set; }

        /// <summary>
        /// </summary>

        [JsonPropertyName("foo_bar")]
        public RootFooBar0 FooBar { get; set; }

    }
}
