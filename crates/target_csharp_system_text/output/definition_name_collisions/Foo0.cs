using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Foo0.JsonConverter))]
    public class Foo0
    {
        public Foo Value { get; set; }
        public class JsonConverter : JsonConverter<Foo0>
        {
            public override Foo0 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
            {
                return new Foo0 { Value = JsonSerializer.Deserialize<Foo>(ref reader, options) };
            }
            public override void Write(Utf8JsonWriter writer, Foo0 value, JsonSerializerOptions options)
            {
                JsonSerializer.Serialize<Foo>(writer, value.Value, options);
            }
        }
    }
}
