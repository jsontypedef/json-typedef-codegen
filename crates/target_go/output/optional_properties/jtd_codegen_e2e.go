package jtd_codegen_e2e

type Root struct {
	Bar []string `json:"bar,omitempty"`

	Baz *bool `json:"baz,omitempty"`

	Foo *string `json:"foo,omitempty"`
}
