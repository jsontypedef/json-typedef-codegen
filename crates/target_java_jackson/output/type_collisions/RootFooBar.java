package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class RootFooBar {
    @JsonProperty("x")
    private Boolean x;

    public RootFooBar() {
    }

    /**
     * Getter for x.<p>
     */
    public Boolean getX() {
        return x;
    }

    /**
     * Setter for x.<p>
     */
    public void setX(Boolean x) {
        this.x = x;
    }
}
