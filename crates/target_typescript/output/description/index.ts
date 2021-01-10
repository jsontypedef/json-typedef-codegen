/**
 * A description for discriminator
 */

export type RootDiscriminatorWithDescription =

	RootDiscriminatorWithDescriptionBar

/**
 * A description for discriminator variant
 */

export interface RootDiscriminatorWithDescriptionBar {
	foo: "bar";

}
/**
 * A description for enum
 */

export enum RootEnumWithDescription {

    /**
     * A description for X
     */

	X = "X",

    /**
     * A description for Y
     */

	Y = "Y",

    /**
     * A description for Z
     */

	Z = "Z",

}
/**
 * A description for properties
 */

export interface RootPropertiesWithDescription {

}
/**

 */

export interface Root {

    /**
     * A description for discriminator
     */

    discriminator_with_description: RootDiscriminatorWithDescription;

    /**
     * A description for enum
     */

    enum_with_description: RootEnumWithDescription;

    /**
     * Whereas disregard and contempt for human rights have resulted in
     * barbarous acts which have outraged the conscience of mankind, and the
     * advent of a world in which human beings shall enjoy freedom of speech and
     * belief and freedom from fear and want has been proclaimed as the highest
     * aspiration of the common people,
     */

    long_description: string;

    /**
     * A description for properties
     */

    properties_with_description: RootPropertiesWithDescription;

    /**
     * A description for ref
     */

    ref_with_description: Baz;

    /**
     * A description for string
     */

    string_with_description: string;

}
/**

 */

export type Baz = string;