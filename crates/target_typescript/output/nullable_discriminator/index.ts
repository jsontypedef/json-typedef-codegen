/**

 */

export type Root0 =

	RootBar

	| RootQuux

/**

 */

export interface RootBar {
	foo: "bar";

    /**

     */

    baz: string;

}
/**

 */

export interface RootQuux {
	foo: "quux";

    /**

     */

    quuz: string;

}
/**

 */

export type Root = (Root0 | null);
