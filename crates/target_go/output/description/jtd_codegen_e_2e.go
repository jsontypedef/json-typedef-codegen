package jtd_codegen_e2e
import "encoding/json"
import "fmt"
type Baz = string
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
		v.Foo = "bar"
		return nil
	}
	return fmt.Errorf("bad Foo value: %s", t.T)
}
type RootDiscriminatorWithDescriptionBar struct {
	Foo string `json:"foo"`
}
type RootEnumWithDescription string
const (
	RootEnumWithDescriptionX RootEnumWithDescription = "X"
	RootEnumWithDescriptionY RootEnumWithDescription = "Y"
	RootEnumWithDescriptionZ RootEnumWithDescription = "Z"
)
type RootPropertiesWithDescription struct {
}
type Root struct {
	DiscriminatorWithDescription RootDiscriminatorWithDescription `json:"discriminator_with_description"`
	EnumWithDescription RootEnumWithDescription `json:"enum_with_description"`
	LongDescription string `json:"long_description"`
	PropertiesWithDescription RootPropertiesWithDescription `json:"properties_with_description"`
	RefWithDescription Baz `json:"ref_with_description"`
	StringWithDescription string `json:"string_with_description"`
}
