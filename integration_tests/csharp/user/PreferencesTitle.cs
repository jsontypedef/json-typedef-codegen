using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Jtd.JtdCodegenDemo
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum PreferencesTitle
    {

        [EnumMember(Value = "MRS")]
        Mrs,

        [EnumMember(Value = "MR")]
        Mr,

        [EnumMember(Value = "HRH")]
        Hrh,

        [EnumMember(Value = "REV")]
        Rev,

        [EnumMember(Value = "MS")]
        Ms,

    }
}
