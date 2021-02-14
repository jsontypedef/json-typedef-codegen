package jtd_codegen_e2e

type Root struct {
	For For `json:"for"`

	Object Object `json:"object"`
}

type For = string

type Object = string
