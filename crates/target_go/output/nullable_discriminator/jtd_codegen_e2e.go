package jtd_codegen_e2e

import (
	"encoding/json"
	"fmt"
)

type Root0 struct {
	Foo string

	Bar RootBar

	Quux RootQuux
}

func (v Root0) MarshalJSON() ([]byte, error) {
	switch v.Foo {
	case "bar":
		return json.Marshal(struct { T string `json:"foo"`; RootBar }{ v.Foo, v.Bar })
	case "quux":
		return json.Marshal(struct { T string `json:"foo"`; RootQuux }{ v.Foo, v.Quux })
	}

	return nil, fmt.Errorf("bad Foo value: %s", v.Foo)
}

func (v *Root0) UnmarshalJSON(b []byte) error {
	var t struct { T string `json:"foo"` }
	if err := json.Unmarshal(b, &t); err != nil {
		return err
	}

	var err error
	switch t.T {
	case "bar":
		err = json.Unmarshal(b, &v.Bar)
	case "quux":
		err = json.Unmarshal(b, &v.Quux)
	default:
		err = fmt.Errorf("bad Foo value: %s", t.T)
	}

	if err != nil {
		return err
	}

	v.Foo = t.T
	return nil
}

type RootBar struct {
	Baz string `json:"baz"`
}

type RootQuux struct {
	Quuz string `json:"quuz"`
}

type Root = *Root0
