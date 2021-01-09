package com.example;


import com.fasterxml.jackson.annotation.JsonInclude;

import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.util.List;

/**

 */

@JsonSerialize

public class RootBar extends Root {

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("baz")
    private List<String> baz;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("quux")
    private Boolean quux;


    public RootBar() {
    }


    public List<String> getBaz() {
        return this.baz;
    }

    public void setBaz(List<String> baz) {
        this.baz = baz;
    }

    public Boolean getQuux() {
        return this.quux;
    }

    public void setQuux(Boolean quux) {
        this.quux = quux;
    }

}
