package com.example;


import com.fasterxml.jackson.annotation.JsonInclude;

import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.util.List;

/**

 */

@JsonSerialize

public class Root {

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bar")
    private List<String> bar;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("baz")
    private Boolean baz;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("foo")
    private String foo;


    public Root() {
    }


    /**

     */

    public List<String> getBar() {
        return this.bar;
    }

    /**

     */

    public void setBar(List<String> bar) {
        this.bar = bar;
    }

    /**

     */

    public Boolean getBaz() {
        return this.baz;
    }

    /**

     */

    public void setBaz(Boolean baz) {
        this.baz = baz;
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

}
