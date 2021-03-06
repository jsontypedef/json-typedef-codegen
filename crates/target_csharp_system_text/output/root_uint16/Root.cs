// Code generated by jtd-codegen for C# + System.Text.Json v0.2.1

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
        public ushort Value { get; set; }
    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new Root { Value = JsonSerializer.Deserialize<ushort>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<ushort>(writer, value.Value, options);
        }
    }
}
