using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Root0.JsonConverter))]
    public class Root0
    {
        public Root? Value { get; set; }
        public class JsonConverter : JsonConverter<Root0>
        {
            public override Root0 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new Root0 { Value = JsonSerializer.Deserialize<Root?>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, Root0 value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<Root?>(writer, value.Value, options);
            }
        }
    }
}
