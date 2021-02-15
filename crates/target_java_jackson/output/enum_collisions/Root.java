package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Root {
    @JsonProperty("foo")
    private RootFoo foo;

    @JsonProperty("foo_bar")
    private RootFooBar0 fooBar;

    public Root() {
    }

    /**
     * Getter for foo.<p>
     */
    public RootFoo getFoo() {
        return foo;
    }

    /**
     * Setter for foo.<p>
     */
    public void setFoo(RootFoo foo) {
        this.foo = foo;
    }

    /**
     * Getter for fooBar.<p>
     */
    public RootFooBar0 getFooBar() {
        return fooBar;
    }

    /**
     * Setter for fooBar.<p>
     */
    public void setFooBar(RootFooBar0 fooBar) {
        this.fooBar = fooBar;
    }
}
