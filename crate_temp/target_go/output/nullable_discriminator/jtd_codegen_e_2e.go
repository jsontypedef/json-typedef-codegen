package jtd_codegen_e2e

import (

	"encoding/json"

	"fmt"

)




type Root0 struct {
	Foo string

	RootBar RootBar

	RootQuux RootQuux

}

func (v Root0) MarshalJSON() ([]byte, error) {
	switch (v.Foo) {

	case "bar":
		return json.Marshal(struct { T string `json:"foo"`; RootBar }{ v.Foo, v.RootBar })

	case "quux":
		return json.Marshal(struct { T string `json:"foo"`; RootQuux }{ v.Foo, v.RootQuux })

	}

	return nil, fmt.Errorf("bad Foo value: %s", v.Foo)
}

func (v *Root0) UnmarshalJSON(b []byte) error {
	var t struct { T string `json:"foo"` }
	if err := json.Unmarshal(b, &t); err != nil {
		return err
	}
	switch t.T {

	case "bar":
		if err := json.Unmarshal(b, &v.RootBar); err != nil {
			return err
		}
		v.Foo = t.T
		return nil

	case "quux":
		if err := json.Unmarshal(b, &v.RootQuux); err != nil {
			return err
		}
		v.Foo = t.T
		return nil

	}

	return fmt.Errorf("bad Foo value: %s", t.T)
}




type RootBar struct {
	Foo string `json:"foo"`





	Baz string `json:"baz"`

}




type RootQuux struct {
	Foo string `json:"foo"`





	Quuz string `json:"quuz"`

}
type Root = *Root0
