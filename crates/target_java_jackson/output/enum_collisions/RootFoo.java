package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class RootFoo {
    @JsonProperty("bar")
    private RootFooBar bar;

    public RootFoo() {
    }

    /**
     * Getter for bar.<p>
     */
    public RootFooBar getBar() {
        return bar;
    }

    /**
     * Setter for bar.<p>
     */
    public void setBar(RootFooBar bar) {
        this.bar = bar;
    }
}
