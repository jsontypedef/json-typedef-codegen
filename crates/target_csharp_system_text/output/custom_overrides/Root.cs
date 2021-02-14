using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("override_elements_container")]
        public System.Collections.Generic.List<string> OverrideElementsContainer { get; set; }

        [JsonPropertyName("override_type_discriminator")]
        public object OverrideTypeDiscriminator { get; set; }

        [JsonPropertyName("override_type_enum")]
        public object OverrideTypeEnum { get; set; }

        [JsonPropertyName("override_type_expr")]
        public object OverrideTypeExpr { get; set; }

        [JsonPropertyName("override_type_properties")]
        public object OverrideTypeProperties { get; set; }

        [JsonPropertyName("override_values_container")]
        public System.Collections.Generic.Dictionary<string, string> OverrideValuesContainer { get; set; }
    }
}
