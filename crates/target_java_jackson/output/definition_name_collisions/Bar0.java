package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

/**

 */

public class Bar0 {
    @JsonValue
    private String value;

    public Bar0() {
    }

    @JsonCreator
    public Bar0(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }
}
