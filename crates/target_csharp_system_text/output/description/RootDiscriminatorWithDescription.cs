using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// A description for discriminator
    /// </summary>
    [JsonConverter(typeof(RootDiscriminatorWithDescriptionJsonConverter))]
    public abstract class RootDiscriminatorWithDescription
    {
    }

    public class RootDiscriminatorWithDescriptionJsonConverter : JsonConverter<RootDiscriminatorWithDescription>
    {
        public override RootDiscriminatorWithDescription Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            var readerCopy = reader;
            var tagValue = JsonDocument.ParseValue(ref reader).RootElement.GetProperty("foo").GetString();

            switch (tagValue)
            {
                case "bar":
                    return JsonSerializer.Deserialize<RootDiscriminatorWithDescriptionBar>(ref readerCopy, options);
                default:
                    throw new ArgumentException(String.Format("Bad Foo value: {0}", tagValue));
            }
        }

        public override void Write(Utf8JsonWriter writer, RootDiscriminatorWithDescription value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize(writer, value, value.GetType(), options);
        }
    }
}
