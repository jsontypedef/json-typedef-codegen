using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    [JsonConverter(typeof(RootJsonConverter))]
    public enum Root
    {

        /// <summary>

        /// </summary>

        Bar,

        /// <summary>

        /// </summary>

        Baz,

        /// <summary>

        /// </summary>

        Foo,

    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {

                case "Bar":
                    return Root.Bar;

                case "Baz":
                    return Root.Baz;

                case "Foo":
                    return Root.Foo;

                default:
                    throw new ArgumentException(String.Format("Bad Root value: {0}", value));
            }
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            switch (value)
            {

                case Root.Bar:
                    JsonSerializer.Serialize<string>(writer, "Bar", options);
                    return;

                case Root.Baz:
                    JsonSerializer.Serialize<string>(writer, "Baz", options);
                    return;

                case Root.Foo:
                    JsonSerializer.Serialize<string>(writer, "Foo", options);
                    return;

            }
        }
    }
}
