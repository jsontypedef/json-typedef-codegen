package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("foo")
    private RootFoo foo;

    
    @JsonProperty("foo_bar")
    private RootFooBar0 fooBar;


    public Root() {
    }


    /**

     */

    public RootFoo getFoo() {
        return this.foo;
    }

    /**

     */

    public void setFoo(RootFoo foo) {
        this.foo = foo;
    }

    /**

     */

    public RootFooBar0 getFooBar() {
        return this.fooBar;
    }

    /**

     */

    public void setFooBar(RootFooBar0 fooBar) {
        this.fooBar = fooBar;
    }

}
