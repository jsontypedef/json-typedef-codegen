package jtd_codegen_e2e

type RootFooBar struct {
	X bool `json:"x"`
}

type RootFoo struct {
	Bar RootFooBar `json:"bar"`
}

type RootFooBar0 struct {
	X string `json:"x"`
}

type Root struct {
	Foo RootFoo `json:"foo"`

	FooBar RootFooBar0 `json:"foo_bar"`
}
