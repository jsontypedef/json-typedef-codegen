using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Bar.JsonConverter))]
    public class Bar
    {
        public Baz Value { get; set; }
        public class JsonConverter : JsonConverter<Bar>
        {
            public override Bar Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new Bar { Value = JsonSerializer.Deserialize<Baz>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, Bar value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<Baz>(writer, value.Value, options);
            }
        }
    }
}
