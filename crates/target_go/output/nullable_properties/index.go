package jtd_codegen_e2e
type Root struct {
	Bar string `json:"bar"`
	Baz []bool `json:"baz"`
	Foo bool `json:"foo"`
	Quux []bool `json:"quux"`
}
type Root0 = *Root
