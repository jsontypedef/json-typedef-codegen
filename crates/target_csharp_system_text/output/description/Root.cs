
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class Root
    {

        /// <summary>
        /// A description for discriminator
        /// </summary>

        [JsonPropertyName("discriminator_with_description")]
        
        public RootDiscriminatorWithDescription DiscriminatorWithDescription { get; set; }

        /// <summary>
        /// A description for enum
        /// </summary>

        [JsonPropertyName("enum_with_description")]
        
        public RootEnumWithDescription EnumWithDescription { get; set; }

        /// <summary>
        /// Whereas disregard and contempt for human rights have resulted in
        /// barbarous acts which have outraged the conscience of mankind, and
        /// the advent of a world in which human beings shall enjoy freedom of
        /// speech and belief and freedom from fear and want has been proclaimed
        /// as the highest aspiration of the common people,
        /// </summary>

        [JsonPropertyName("long_description")]
        
        public string LongDescription { get; set; }

        /// <summary>
        /// A description for properties
        /// </summary>

        [JsonPropertyName("properties_with_description")]
        
        public RootPropertiesWithDescription PropertiesWithDescription { get; set; }

        /// <summary>
        /// A description for ref
        /// </summary>

        [JsonPropertyName("ref_with_description")]
        
        public Baz RefWithDescription { get; set; }

        /// <summary>
        /// A description for string
        /// </summary>

        [JsonPropertyName("string_with_description")]
        
        public string StringWithDescription { get; set; }

    }
}
