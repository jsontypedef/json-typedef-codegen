package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class RootFooBar {

    
    @JsonProperty("x")
    private Boolean x;


    public RootFooBar() {
    }


    /**

     */

    public Boolean getX() {
        return this.x;
    }

    /**

     */

    public void setX(Boolean x) {
        this.x = x;
    }

}
