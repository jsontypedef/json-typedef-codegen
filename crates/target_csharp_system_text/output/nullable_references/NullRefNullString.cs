using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(NullRefNullString.JsonConverter))]
    public class NullRefNullString
    {
        public NullString? Value { get; set; }
        public class JsonConverter : JsonConverter<NullRefNullString>
        {
            public override NullRefNullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new NullRefNullString { Value = JsonSerializer.Deserialize<NullString?>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, NullRefNullString value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<NullString?>(writer, value.Value, options);
            }
        }
    }
}
