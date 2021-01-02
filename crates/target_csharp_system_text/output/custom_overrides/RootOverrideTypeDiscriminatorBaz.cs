
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class RootOverrideTypeDiscriminatorBaz : object
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "baz"; }

    }
}
