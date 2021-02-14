package jtd_codegen_e2e

import (
	"encoding/json"
	"fmt"
)

type Root struct {
	Foo string

	BarBaz RootBarBaz

	Quux RootQuux
}

func (v Root) MarshalJSON() ([]byte, error) {
	switch v.Foo {
	case "BAR_BAZ":
		return json.Marshal(struct { T string `json:"foo"`; RootBarBaz }{ v.Foo, v.BarBaz })
	case "QUUX":
		return json.Marshal(struct { T string `json:"foo"`; RootQuux }{ v.Foo, v.Quux })
	}

	return nil, fmt.Errorf("bad Foo value: %s", v.Foo)
}

func (v *Root) UnmarshalJSON(b []byte) error {
	var t struct { T string `json:"foo"` }
	if err := json.Unmarshal(b, &t); err != nil {
		return err
	}

	var err error
	switch t.T {
	case "BAR_BAZ":
		err = json.Unmarshal(b, &v.BarBaz)
	case "QUUX":
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

type RootBarBaz struct {
	Baz string `json:"baz"`
}

type RootQuux struct {
	Quuz string `json:"quuz"`
}
