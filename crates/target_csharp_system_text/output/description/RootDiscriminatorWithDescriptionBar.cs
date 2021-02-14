using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// A description for discriminator variant
    /// </summary>
    public class RootDiscriminatorWithDescriptionBar : RootDiscriminatorWithDescription
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "bar"; }
    }
}
