// Code generated by jtd-codegen for Java + Jackson v0.2.1

package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class RootQuux extends Root {
    @JsonProperty("quuz")
    private String quuz;

    public RootQuux() {
    }

    /**
     * Getter for quuz.<p>
     */
    public String getQuuz() {
        return quuz;
    }

    /**
     * Setter for quuz.<p>
     */
    public void setQuuz(String quuz) {
        this.quuz = quuz;
    }
}
