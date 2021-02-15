package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * A description for a definition
 */
public class Baz {
    @JsonValue
    private String value;

    public Baz() {
    }

    @JsonCreator
    public Baz(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }
}
