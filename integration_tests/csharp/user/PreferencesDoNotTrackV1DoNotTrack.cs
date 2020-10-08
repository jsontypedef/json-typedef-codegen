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

        /// <value>
        /// All forms of tracking permitted.
        /// </value>
        [EnumMember(Value = "ALL")]
        All,

        /// <value>
        /// Only essentialy forms of tracking permitted.
        /// </value>
        [EnumMember(Value = "ESSENTIAL_ONLY")]
        EssentialOnly,

        /// <value>
        /// No forms forms of tracking permitted.
        /// </value>
        [EnumMember(Value = "NONE")]
        None,

    }
}
