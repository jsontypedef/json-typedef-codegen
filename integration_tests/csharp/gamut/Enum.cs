using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Jtd.JtdCodegenDemo
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum Enum
    {

        [EnumMember(Value = "bar")]
        Bar,

        [EnumMember(Value = "foo")]
        Foo,

        [EnumMember(Value = "baz")]
        Baz,

    }
}
