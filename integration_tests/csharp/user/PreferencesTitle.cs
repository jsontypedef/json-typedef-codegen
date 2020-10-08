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

        /// <value>
        /// Refer to this user as 'His/Her Royal Highness'
        /// </value>
        [EnumMember(Value = "HRH")]
        Hrh,

        /// <value>
        /// Refer to this user as 'Mr.'
        /// </value>
        [EnumMember(Value = "MR")]
        Mr,

        /// <value>
        /// Refer to this user as 'Mrs.'
        /// </value>
        [EnumMember(Value = "MRS")]
        Mrs,

        /// <value>
        /// Refer to this user as 'Ms.'
        /// </value>
        [EnumMember(Value = "MS")]
        Ms,

        /// <value>
        /// Refer to this user as 'Rev.'
        /// </value>
        [EnumMember(Value = "REV")]
        Rev,

    }
}
