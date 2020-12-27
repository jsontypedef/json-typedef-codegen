package jtd_codegen_e2e

import (

	"encoding/json"

	"fmt"

)
type Baz = string

// A description for discriminator


type RootDiscriminatorWithDescription struct {
	Foo string

	RootDiscriminatorWithDescriptionBar RootDiscriminatorWithDescriptionBar

}

func (v RootDiscriminatorWithDescription) MarshalJSON() ([]byte, error) {
	switch (v.Foo) {

	case "bar":
		return json.Marshal(struct { T string `json:"foo"`; RootDiscriminatorWithDescriptionBar }{ v.Foo, v.RootDiscriminatorWithDescriptionBar })

	}

	return nil, fmt.Errorf("bad Foo value: %s", v.Foo)
}

func (v *RootDiscriminatorWithDescription) UnmarshalJSON(b []byte) error {
	var t struct { T string `json:"foo"` }
	if err := json.Unmarshal(b, &t); err != nil {
		return err
	}
	switch t.T {

	case "bar":
		if err := json.Unmarshal(b, &v.RootDiscriminatorWithDescriptionBar); err != nil {
			return err
		}
		v.Foo = t.T
		return nil

	}

	return fmt.Errorf("bad Foo value: %s", t.T)
}

// A description for discriminator variant


type RootDiscriminatorWithDescriptionBar struct {
	Foo string `json:"foo"`

}

    // A description for enum


type RootEnumWithDescription string

const (


    // A description for X


	RootEnumWithDescriptionX RootEnumWithDescription = "X"


    // A description for Y


	RootEnumWithDescriptionY RootEnumWithDescription = "Y"


    // A description for Z


	RootEnumWithDescriptionZ RootEnumWithDescription = "Z"

)

// A description for properties


type RootPropertiesWithDescription struct {

}




type Root struct {


    // A description for discriminator


    DiscriminatorWithDescription RootDiscriminatorWithDescription `json:"discriminator_with_description"`


    // A description for enum


    EnumWithDescription RootEnumWithDescription `json:"enum_with_description"`


    // Whereas disregard and contempt for human rights have resulted in
    // barbarous acts which have outraged the conscience of mankind, and the
    // advent of a world in which human beings shall enjoy freedom of speech and
    // belief and freedom from fear and want has been proclaimed as the highest
    // aspiration of the common people,


    LongDescription string `json:"long_description"`


    // A description for properties


    PropertiesWithDescription RootPropertiesWithDescription `json:"properties_with_description"`


    // A description for ref


    RefWithDescription Baz `json:"ref_with_description"`


    // A description for string


    StringWithDescription string `json:"string_with_description"`

}
