package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Root {
    @JsonValue
    private Foo0 value;

    public Root() {
    }

    @JsonCreator
    public Root(Foo0 value) {
        this.value = value;
    }

    public Foo0 getValue() {
        return value;
    }

    public void setValue(Foo0 value) {
        this.value = value;
    }
}
