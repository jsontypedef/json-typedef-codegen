package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class RootBar extends Root {
    @JsonProperty("baz")
    private String baz;

    public RootBar() {
    }

    /**
     * Getter for baz.<p>
     */
    public String getBaz() {
        return baz;
    }

    /**
     * Setter for baz.<p>
     */
    public void setBaz(String baz) {
        this.baz = baz;
    }
}
