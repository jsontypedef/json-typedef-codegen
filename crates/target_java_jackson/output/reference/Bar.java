package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Bar {
    @JsonValue
    private Baz value;

    public Bar() {
    }

    @JsonCreator
    public Bar(Baz value) {
        this.value = value;
    }

    public Baz getValue() {
        return value;
    }

    public void setValue(Baz value) {
        this.value = value;
    }
}
