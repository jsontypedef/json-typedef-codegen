// Code generated by jtd-codegen for C# + System.Text.Json v0.2.1

using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    [JsonConverter(typeof(RootJsonConverter))]
    public enum Root
    {
        Bar,

        Baz,

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
