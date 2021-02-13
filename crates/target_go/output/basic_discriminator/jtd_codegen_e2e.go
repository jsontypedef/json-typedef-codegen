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
	switch (v.Foo) {

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
	switch t.T {

	case "BAR_BAZ":
		if err := json.Unmarshal(b, &v.BarBaz); err != nil {
			return err
		}
		v.Foo = t.T
		return nil

	case "QUUX":
		if err := json.Unmarshal(b, &v.Quux); err != nil {
			return err
		}
		v.Foo = t.T
		return nil

	}

	return fmt.Errorf("bad Foo value: %s", t.T)
}




type RootBarBaz struct {
	Foo string `json:"foo"`





	Baz string `json:"baz"`

}




type RootQuux struct {
	Foo string `json:"foo"`





	Quuz string `json:"quuz"`

}
