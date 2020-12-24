using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NullString.JsonConverter))]
    public class NullString
    {
        public string Value { get; set; }
        public class JsonConverter : JsonConverter<NullString>
        {
            public override NullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NullString { Value = JsonSerializer.Deserialize<string>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<string>(writer, value.Value, options);
            }
        }
    }
}
