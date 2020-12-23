using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Baz.JsonConverter))]
    public class Baz
    {
        public string Value { get; set; }
        public class JsonConverter : JsonConverter<Baz>
        {
            public override Baz Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new Baz { Value = JsonSerializer.Deserialize<string>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, Baz value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<string>(writer, value.Value, options);
            }
        }
    }
}
