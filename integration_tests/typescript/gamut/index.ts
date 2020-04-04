
export type Discriminator = DiscriminatorBar | DiscriminatorBaz;

export type Elements = Element[];

export type Empty = any;

export type Enum = ("bar" | "baz" | "foo");

export type Values = {[name: string]: Value};


export interface Element {

    elementThing: any;
}


export interface DiscriminatorBar {

    barThing: any;

    foo: "bar";
}


export interface DiscriminatorBaz {

    bazThing: any;

    foo: "baz";
}


export interface Type {

    uint32: number;

    float32: number;

    string: string;

    timestamp: string;

    float64: number;

    uint16: number;

    uint8: number;

    int8: number;

    int32: number;

    int16: number;

    boolean: boolean;
}


export interface Value {

    valueThing: any;
}


export interface Gamut {

    values: Values;

    enum: Enum;

    elements: Elements;

    discriminator: Discriminator;

    empty: Empty;

    type: Type;
}

