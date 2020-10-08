using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// A title we should use when addressing the user formally.
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum PreferencesTitle
    {

        /// <summary>
        /// Refer to this user as 'His/Her Royal Highness'
        /// </summary>
        [EnumMember(Value = "HRH")]
        Hrh,

        /// <summary>
        /// Refer to this user as 'Mr.'
        /// </summary>
        [EnumMember(Value = "MR")]
        Mr,

        /// <summary>
        /// Refer to this user as 'Mrs.'
        /// </summary>
        [EnumMember(Value = "MRS")]
        Mrs,

        /// <summary>
        /// Refer to this user as 'Ms.'
        /// </summary>
        [EnumMember(Value = "MS")]
        Ms,

        /// <summary>
        /// Refer to this user as 'Rev.'
        /// </summary>
        [EnumMember(Value = "REV")]
        Rev,

    }
}
