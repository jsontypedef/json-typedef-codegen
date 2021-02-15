package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Root {
    @JsonValue
    private Foo value;

    public Root() {
    }

    @JsonCreator
    public Root(Foo value) {
        this.value = value;
    }

    public Foo getValue() {
        return value;
    }

    public void setValue(Foo value) {
        this.value = value;
    }
}
