using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// A multi-level do-not-track setting
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum PreferencesDoNotTrackV1DoNotTrack
    {

        /// <summary>
        /// All forms of tracking permitted.
        /// </summary>
        [EnumMember(Value = "ALL")]
        All,

        /// <summary>
        /// Only essentialy forms of tracking permitted.
        /// </summary>
        [EnumMember(Value = "ESSENTIAL_ONLY")]
        EssentialOnly,

        /// <summary>
        /// No forms forms of tracking permitted.
        /// </summary>
        [EnumMember(Value = "NONE")]
        None,

    }
}
