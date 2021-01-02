
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class Root
    {

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_elements_container")]
        public System.Collections.Generic.List<string> OverrideElementsContainer { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_type_discriminator")]
        public object OverrideTypeDiscriminator { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_type_enum")]
        public object OverrideTypeEnum { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_type_expr")]
        public object OverrideTypeExpr { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_type_properties")]
        public object OverrideTypeProperties { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("override_values_container")]
        public System.Collections.Generic.Dictionary<string, string> OverrideValuesContainer { get; set; }

    }
}
