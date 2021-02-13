package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class RootBarBaz extends Root {

    
    @JsonProperty("baz")
    private String baz;


    public RootBarBaz() {
    }


    public String getBaz() {
        return this.baz;
    }

    public void setBaz(String baz) {
        this.baz = baz;
    }

}
