
using System;

using System.Text.Json;

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>
    /// A description for enum
    /// </summary>

    [JsonConverter(typeof(RootEnumWithDescriptionJsonConverter))]
    public enum RootEnumWithDescription
    {

        /// <summary>
        /// A description for X
        /// </summary>

        X,

        /// <summary>
        /// A description for Y
        /// </summary>

        Y,

        /// <summary>
        /// A description for Z
        /// </summary>

        Z,

    }

    public class RootEnumWithDescriptionJsonConverter : JsonConverter<RootEnumWithDescription>
    {
        public override RootEnumWithDescription Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {

                case "X":
                    return RootEnumWithDescription.X;

                case "Y":
                    return RootEnumWithDescription.Y;

                case "Z":
                    return RootEnumWithDescription.Z;

                default:
                    throw new ArgumentException(String.Format("Bad RootEnumWithDescription value: {0}", value));
            }
        }

        public override void Write(Utf8JsonWriter writer, RootEnumWithDescription value, JsonSerializerOptions options)
        {
            switch (value)
            {

                case RootEnumWithDescription.X:
                    JsonSerializer.Serialize<string>(writer, "X", options);
                    return;

                case RootEnumWithDescription.Y:
                    JsonSerializer.Serialize<string>(writer, "Y", options);
                    return;

                case RootEnumWithDescription.Z:
                    JsonSerializer.Serialize<string>(writer, "Z", options);
                    return;

            }
        }
    }
}
