using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NotnullString.JsonConverter))]
    public class NotnullString
    {
        public string Value { get; set; }
        public class JsonConverter : JsonConverter<NotnullString>
        {
            public override NotnullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NotnullString { Value = JsonSerializer.Deserialize<string>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NotnullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<string>(writer, value.Value, options);
            }
        }
    }
}
