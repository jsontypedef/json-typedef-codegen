package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Root {
    @JsonValue
    private Byte value;

    public Root() {
    }

    @JsonCreator
    public Root(Byte value) {
        this.value = value;
    }

    public Byte getValue() {
        return value;
    }

    public void setValue(Byte value) {
        this.value = value;
    }
}
