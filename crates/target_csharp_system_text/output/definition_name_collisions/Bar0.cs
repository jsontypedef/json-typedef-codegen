using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Bar0JsonConverter))]
    public class Bar0
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public string Value { get; set; }
    }

    public class Bar0JsonConverter : JsonConverter<Bar0>
    {
        public override Bar0 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new Bar0 { Value = JsonSerializer.Deserialize<string>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, Bar0 value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<string>(writer, value.Value, options);
        }
    }
}
