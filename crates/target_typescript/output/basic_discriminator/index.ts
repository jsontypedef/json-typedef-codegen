/**

 */

export type Root =

	RootBarBaz

	| RootQuux

/**

 */

export interface RootBarBaz {
	foo: "BAR_BAZ";

    /**

     */

    baz: string;

}
/**

 */

export interface RootQuux {
	foo: "QUUX";

    /**

     */

    quuz: string;

}
