/**

 */

export interface RootFooBar {

    /**

     */

    x: boolean;

}
/**

 */

export interface RootFoo {

    /**

     */

    bar: RootFooBar;

}
/**

 */

export interface RootFooBar0 {

    /**

     */

    x: string;

}
/**

 */

export interface Root {

    /**

     */

    foo: RootFoo;

    /**

     */

    foo_bar: RootFooBar0;

}
