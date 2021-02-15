package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Foo0 {
    @JsonValue
    private Foo value;

    public Foo0() {
    }

    @JsonCreator
    public Foo0(Foo value) {
        this.value = value;
    }

    public Foo getValue() {
        return value;
    }

    public void setValue(Foo value) {
        this.value = value;
    }
}
