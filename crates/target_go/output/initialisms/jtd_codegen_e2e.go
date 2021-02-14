package jtd_codegen_e2e

type RootNestedIDInitialism struct {
	JSON string `json:"json"`

	Normalword string `json:"normalword"`
}

type Root struct {
	HTTP string `json:"http"`

	ID string `json:"id"`

	NestedIDInitialism RootNestedIDInitialism `json:"nested_id_initialism"`

	UTF8 string `json:"utf8"`

	WordWithEmbeddedIDInitialism string `json:"word_with_embedded_id_initialism"`

	WordWithTrailingInitialismID string `json:"word_with_trailing_initialism_id"`
}
