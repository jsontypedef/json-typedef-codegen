package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class RootQuux extends Root {

    
    @JsonProperty("quuz")
    private String quuz;


    public RootQuux() {
    }


    public String getQuuz() {
        return this.quuz;
    }

    public void setQuuz(String quuz) {
        this.quuz = quuz;
    }

}
