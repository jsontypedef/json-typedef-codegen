using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    [JsonConverter(typeof(RootFooBarJsonConverter))]
    public enum RootFooBar
    {
        X,

        Y,
    }
    public class RootFooBarJsonConverter : JsonConverter<RootFooBar>
    {
        public override RootFooBar Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {
                case "x":
                    return RootFooBar.X;
                case "y":
                    return RootFooBar.Y;
                default:
                    throw new ArgumentException(String.Format("Bad RootFooBar value: {0}", value));
            }
        }

        public override void Write(Utf8JsonWriter writer, RootFooBar value, JsonSerializerOptions options)
        {
            switch (value)
            {
                case RootFooBar.X:
                    JsonSerializer.Serialize<string>(writer, "x", options);
                    return;
                case RootFooBar.Y:
                    JsonSerializer.Serialize<string>(writer, "y", options);
                    return;
            }
        }
    }
}
