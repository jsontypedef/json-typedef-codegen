
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{

    public class RootNestedIdInitialism
    {


        [JsonPropertyName("json")]
        
        public string Json { get; set; }


        [JsonPropertyName("normalword")]
        
        public string Normalword { get; set; }

    }
}
