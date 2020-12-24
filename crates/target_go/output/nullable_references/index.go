package jtd_codegen_e2e
type NotnullRefNotnullString = NotnullString
type NotnullRefNullString = NullString
type NotnullString = string
type NullRefNotnullString = *NotnullString
type NullRefNullString = *NullString
type NullString = *string
type Root struct {
	NotnullRefNotnullString NotnullRefNotnullString `json:"notnull_ref_notnull_string"`
	NotnullRefNullString NotnullRefNullString `json:"notnull_ref_null_string"`
	NotnullString NotnullString `json:"notnull_string"`
	NullRefNotnullString NullRefNotnullString `json:"null_ref_notnull_string"`
	NullRefNullString NullRefNullString `json:"null_ref_null_string"`
	NullString NullString `json:"null_string"`
}
