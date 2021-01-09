package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class RootFoo {

    
    @JsonProperty("bar")
    private RootFooBar bar;


    public RootFoo() {
    }


    /**

     */

    public RootFooBar getBar() {
        return this.bar;
    }

    /**

     */

    public void setBar(RootFooBar bar) {
        this.bar = bar;
    }

}
