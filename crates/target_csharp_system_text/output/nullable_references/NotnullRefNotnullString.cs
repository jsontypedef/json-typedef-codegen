using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NotnullRefNotnullString.JsonConverter))]
    public class NotnullRefNotnullString
    {
        public NotnullString Value { get; set; }
        public class JsonConverter : JsonConverter<NotnullRefNotnullString>
        {
            public override NotnullRefNotnullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NotnullRefNotnullString { Value = JsonSerializer.Deserialize<NotnullString>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NotnullRefNotnullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<NotnullString>(writer, value.Value, options);
            }
        }
    }
}
