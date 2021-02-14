using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootOverrideTypeDiscriminatorBaz : object
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "baz"; }
    }
}
