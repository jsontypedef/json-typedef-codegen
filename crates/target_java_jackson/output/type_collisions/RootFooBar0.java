package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class RootFooBar0 {

    
    @JsonProperty("x")
    private String x;


    public RootFooBar0() {
    }


    /**

     */

    public String getX() {
        return this.x;
    }

    /**

     */

    public void setX(String x) {
        this.x = x;
    }

}
