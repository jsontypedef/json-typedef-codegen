package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("Foo")
    private String foo;

    
    @JsonProperty("foo")
    private String foo0;


    public Root() {
    }


    /**

     */

    public String getFoo() {
        return this.foo;
    }

    /**

     */

    public void setFoo(String foo) {
        this.foo = foo;
    }

    /**

     */

    public String getFoo0() {
        return this.foo0;
    }

    /**

     */

    public void setFoo0(String foo0) {
        this.foo0 = foo0;
    }

}
