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

        Foo,

        /// <summary>
        /// </summary>

        Foo0,

        /// <summary>
        /// </summary>

        Foo1,

    }

    public class RootJsonConverter : JsonConverter<Root>
    {
        public override Root Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {

                case "FOO":
                    return Root.Foo;

                case "Foo":
                    return Root.Foo0;

                case "foo":
                    return Root.Foo1;

                default:
                    throw new ArgumentException(String.Format("Bad Root value: {0}", value));
            }
        }

        public override void Write(Utf8JsonWriter writer, Root value, JsonSerializerOptions options)
        {
            switch (value)
            {

                case Root.Foo:
                    JsonSerializer.Serialize<string>(writer, "FOO", options);
                    return;

                case Root.Foo0:
                    JsonSerializer.Serialize<string>(writer, "Foo", options);
                    return;

                case Root.Foo1:
                    JsonSerializer.Serialize<string>(writer, "foo", options);
                    return;

            }
        }
    }
}
