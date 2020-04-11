
export type Discriminator = DiscriminatorBar | DiscriminatorBaz;


export type Elements = Element[];


export type Empty = any;


export type Enum = ("bar" | "baz" | "foo");


export type Values = {[name: string]: Value};


export interface Type {

  boolean: boolean;

  float32: number;

  float64: number;

  int16: number;

  int32: number;

  int8: number;

  string: string;

  timestamp: string;

  uint16: number;

  uint32: number;

  uint8: number;
}


export interface Element {

  elementThing: any;
}


export interface Value {

  valueThing: any;
}


export interface DiscriminatorBar {

  barThing: any;

  foo: "bar";
}


export interface DiscriminatorBaz {

  bazThing: any;

  foo: "baz";
}


export interface Gamut {

  discriminator: Discriminator;

  elements: Elements;

  empty: Empty;

  enum: Enum;

  type: Type;

  values: Values;
}

