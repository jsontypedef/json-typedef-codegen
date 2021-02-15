package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Root {
    @JsonProperty("Foo")
    private String foo;

    @JsonProperty("foo")
    private String foo0;

    public Root() {
    }

    /**
     * Getter for foo.<p>
     */
    public String getFoo() {
        return foo;
    }

    /**
     * Setter for foo.<p>
     */
    public void setFoo(String foo) {
        this.foo = foo;
    }

    /**
     * Getter for foo0.<p>
     */
    public String getFoo0() {
        return foo0;
    }

    /**
     * Setter for foo0.<p>
     */
    public void setFoo0(String foo0) {
        this.foo0 = foo0;
    }
}
