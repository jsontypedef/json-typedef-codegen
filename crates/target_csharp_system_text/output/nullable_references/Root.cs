
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    /// <summary>

    /// </summary>

    public class Root
    {

        /// <summary>

        /// </summary>

        [JsonPropertyName("notnull_ref_notnull_string")]
        
        public NotnullRefNotnullString NotnullRefNotnullString { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("notnull_ref_null_string")]
        
        public NotnullRefNullString NotnullRefNullString { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("notnull_string")]
        
        public NotnullString NotnullString { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("null_ref_notnull_string")]
        
        public NullRefNotnullString NullRefNotnullString { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("null_ref_null_string")]
        
        public NullRefNullString NullRefNullString { get; set; }

        /// <summary>

        /// </summary>

        [JsonPropertyName("null_string")]
        
        public NullString NullString { get; set; }

    }
}
