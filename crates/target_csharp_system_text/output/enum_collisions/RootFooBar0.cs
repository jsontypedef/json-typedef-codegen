using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>
    /// </summary>

    [JsonConverter(typeof(RootFooBar0JsonConverter))]
    public enum RootFooBar0
    {

        /// <summary>
        /// </summary>

        X,

        /// <summary>
        /// </summary>

        Y,

    }

    public class RootFooBar0JsonConverter : JsonConverter<RootFooBar0>
    {
        public override RootFooBar0 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {

                case "x":
                    return RootFooBar0.X;

                case "y":
                    return RootFooBar0.Y;

                default:
                    throw new ArgumentException(String.Format("Bad RootFooBar0 value: {0}", value));
            }
        }

        public override void Write(Utf8JsonWriter writer, RootFooBar0 value, JsonSerializerOptions options)
        {
            switch (value)
            {

                case RootFooBar0.X:
                    JsonSerializer.Serialize<string>(writer, "x", options);
                    return;

                case RootFooBar0.Y:
                    JsonSerializer.Serialize<string>(writer, "y", options);
                    return;

            }
        }
    }
}
