using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Root.JsonConverter))]
    public class Root
    {
        public string Value { get; set; }
        public class JsonConverter : JsonConverter<Root>
        {
            public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new Root { Value = JsonSerializer.Deserialize<string>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<string>(writer, value.Value, options);
            }
        }
    }
}
