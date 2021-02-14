using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    [JsonConverter(typeof(RootJsonConverter))]
    public class Root
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public short Value { get; set; }
    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new Root { Value = JsonSerializer.Deserialize<short>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<short>(writer, value.Value, options);
        }
    }
}
