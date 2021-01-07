
using System;

using System.Text.Json;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    [JsonConverter(typeof(NotnullRefNotnullStringJsonConverter))]
    public class NotnullRefNotnullString
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public NotnullString Value { get; set; }
    }

    public class NotnullRefNotnullStringJsonConverter : JsonConverter<NotnullRefNotnullString>
    {
        public override NotnullRefNotnullString Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new NotnullRefNotnullString { Value = JsonSerializer.Deserialize<NotnullString>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, NotnullRefNotnullString value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<NotnullString>(writer, value.Value, options);
        }
    }
}
