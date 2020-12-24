using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NotnullRefNullString.JsonConverter))]
    public class NotnullRefNullString
    {
        public NullString Value { get; set; }
        public class JsonConverter : JsonConverter<NotnullRefNullString>
        {
            public override NotnullRefNullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NotnullRefNullString { Value = JsonSerializer.Deserialize<NullString>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NotnullRefNullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<NullString>(writer, value.Value, options);
            }
        }
    }
}
