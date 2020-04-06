package gamut


import "time"

import "encoding/json"



type DiscriminatorFoo string

type Elements []Element

type Empty interface{}

type Enum string

type Values map[string]Value

const DiscriminatorFooBar DiscriminatorFoo = "bar"

const DiscriminatorFooBaz DiscriminatorFoo = "baz"

const EnumEnumBar Enum = "bar"

const EnumEnumBaz Enum = "baz"

const EnumEnumFoo Enum = "foo"


type DiscriminatorBar struct {

  BarThing interface{} `json:"barThing"`
}


type DiscriminatorBaz struct {

  BazThing interface{} `json:"bazThing"`
}


type Element struct {

  ElementThing interface{} `json:"elementThing"`
}


type Gamut struct {

  Discriminator Discriminator `json:"discriminator"`

  Elements Elements `json:"elements"`

  Empty Empty `json:"empty"`

  Enum Enum `json:"enum"`

  Type Type `json:"type"`

  Values Values `json:"values"`
}


type Type struct {

  Boolean bool `json:"boolean"`

  Float32 float32 `json:"float32"`

  Float64 float64 `json:"float64"`

  Int16 int16 `json:"int16"`

  Int32 int32 `json:"int32"`

  Int8 int8 `json:"int8"`

  String string `json:"string"`

  Timestamp time.Time `json:"timestamp"`

  Uint16 uint16 `json:"uint16"`

  Uint32 uint32 `json:"uint32"`

  Uint8 uint8 `json:"uint8"`
}


type Value struct {

  ValueThing interface{} `json:"valueThing"`
}


