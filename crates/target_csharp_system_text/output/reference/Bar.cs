// Code generated by jtd-codegen for C# + System.Text.Json v0.2.0

using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    [JsonConverter(typeof(BarJsonConverter))]
    public class Bar
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public Baz Value { get; set; }
    }

    public class BarJsonConverter : JsonConverter<Bar>
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
