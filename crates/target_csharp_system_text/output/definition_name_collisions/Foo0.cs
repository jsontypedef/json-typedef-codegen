using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    [JsonConverter(typeof(Foo0JsonConverter))]
    public class Foo0
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public Foo Value { get; set; }
    }

    public class Foo0JsonConverter : JsonConverter<Foo0>
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
