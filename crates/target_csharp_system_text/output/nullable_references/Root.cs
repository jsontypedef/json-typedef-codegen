using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    public class Root
    {
        [JsonPropertyName("notnull_ref_notnull_string")]
        public NotnullRefNotnullString NotnullRefNotnullString { get; set; }
        [JsonPropertyName("notnull_ref_null_string")]
        public NotnullRefNullString NotnullRefNullString { get; set; }
        [JsonPropertyName("notnull_string")]
        public NotnullString NotnullString { get; set; }
        [JsonPropertyName("null_ref_notnull_string")]
        public NullRefNotnullString NullRefNotnullString { get; set; }
        [JsonPropertyName("null_ref_null_string")]
        public NullRefNullString NullRefNullString { get; set; }
        [JsonPropertyName("null_string")]
        public NullString NullString { get; set; }
    }
}
