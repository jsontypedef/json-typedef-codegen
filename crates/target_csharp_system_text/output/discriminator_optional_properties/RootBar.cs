// Code generated by jtd-codegen for C# + System.Text.Json v0.2.1

using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootBar : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "bar"; }

        [JsonPropertyName("baz")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public IList<string> Baz { get; set; }

        [JsonPropertyName("quux")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public bool? Quux { get; set; }
    }
}
