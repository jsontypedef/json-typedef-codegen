using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
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
                case "bar":
                    return JsonSerializer.Deserialize<RootBar>(ref readerCopy, options);
                case "quux":
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
