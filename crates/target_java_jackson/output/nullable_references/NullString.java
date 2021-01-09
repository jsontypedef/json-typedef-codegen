package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

/**

 */

public class NullString {
    @JsonValue
    private String value;

    public NullString() {
    }

    @JsonCreator
    public NullString(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }
}
