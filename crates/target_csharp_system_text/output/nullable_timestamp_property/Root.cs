using System;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("foo")]
        public DateTimeOffset? Foo { get; set; }
    }
}
