using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>
    /// </summary>

    [JsonConverter(typeof(RootJsonConverter))]
    public class Root
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public IList<string> Value { get; set; }
    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new Root { Value = JsonSerializer.Deserialize<IList<string>>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<IList<string>>(writer, value.Value, options);
        }
    }
}
