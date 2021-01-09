package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.util.List;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("bar")
    private String bar;

    
    @JsonProperty("baz")
    private List<Boolean> baz;

    
    @JsonProperty("foo")
    private Boolean foo;

    
    @JsonProperty("quux")
    private List<Boolean> quux;


    public Root() {
    }


    /**

     */

    public String getBar() {
        return this.bar;
    }

    /**

     */

    public void setBar(String bar) {
        this.bar = bar;
    }

    /**

     */

    public List<Boolean> getBaz() {
        return this.baz;
    }

    /**

     */

    public void setBaz(List<Boolean> baz) {
        this.baz = baz;
    }

    /**

     */

    public Boolean getFoo() {
        return this.foo;
    }

    /**

     */

    public void setFoo(Boolean foo) {
        this.foo = foo;
    }

    /**

     */

    public List<Boolean> getQuux() {
        return this.quux;
    }

    /**

     */

    public void setQuux(List<Boolean> quux) {
        this.quux = quux;
    }

}
