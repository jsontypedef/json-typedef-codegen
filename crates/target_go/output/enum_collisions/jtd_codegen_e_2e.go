package jtd_codegen_e2e

import (

)




type RootFooBar string

const (





	RootFooBarX RootFooBar = "x"





	RootFooBarY RootFooBar = "y"

)




type RootFoo struct {





    Bar RootFooBar `json:"bar"`

}




type RootFooBar0 string

const (





	RootFooBarX0 RootFooBar0 = "x"





	RootFooBarY0 RootFooBar0 = "y"

)




type Root struct {





    Foo RootFoo `json:"foo"`





    FooBar RootFooBar0 `json:"foo_bar"`

}
