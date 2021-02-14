package jtd_codegen_e2e

type RootOverrideTypeDiscriminatorBaz struct {
}

type Root struct {
	OverrideElementsContainer []string `json:"override_elements_container"`

	OverrideTypeDiscriminator interface{} `json:"override_type_discriminator"`

	OverrideTypeEnum interface{} `json:"override_type_enum"`

	OverrideTypeExpr interface{} `json:"override_type_expr"`

	OverrideTypeProperties interface{} `json:"override_type_properties"`

	OverrideValuesContainer map[string]string `json:"override_values_container"`
}
