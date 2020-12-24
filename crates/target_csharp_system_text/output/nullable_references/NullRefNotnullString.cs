using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NullRefNotnullString.JsonConverter))]
    public class NullRefNotnullString
    {
        public NotnullString? Value { get; set; }
        public class JsonConverter : JsonConverter<NullRefNotnullString>
        {
            public override NullRefNotnullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NullRefNotnullString { Value = JsonSerializer.Deserialize<NotnullString?>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NullRefNotnullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<NotnullString?>(writer, value.Value, options);
            }
        }
    }
}
