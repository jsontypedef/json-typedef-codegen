using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Jtd.JtdCodegenDemo
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum PreferencesDoNotTrackV1DoNotTrack
    {

        [EnumMember(Value = "ALL")]
        All,

        [EnumMember(Value = "NONE")]
        None,

        [EnumMember(Value = "ESSENTIAL_ONLY")]
        EssentialOnly,

    }
}
