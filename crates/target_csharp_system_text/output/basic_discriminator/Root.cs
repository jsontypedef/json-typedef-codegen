
using System;

using System.Text.Json;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    [JsonConverter(typeof(RootJsonConverter))]
    public abstract class Root
    {
    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            var readerCopy = reader;
            var tagValue = JsonDocument.ParseValue(ref reader).RootElement.GetProperty("foo").GetString();

            switch (tagValue)
            {
                case "BAR_BAZ":
                    return JsonSerializer.Deserialize<RootBarBaz>(ref readerCopy, options);
                case "QUUX":
                    return JsonSerializer.Deserialize<RootQuux>(ref readerCopy, options);
                default:
                    throw new ArgumentException(String.Format("Bad Foo value: {0}", tagValue));
            }
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize(writer, value, value.GetType(), options);
        }
    }
}
