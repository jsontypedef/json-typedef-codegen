package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

/**

 */

public class Foo {
    @JsonValue
    private Bar value;

    public Foo() {
    }

    @JsonCreator
    public Foo(Bar value) {
        this.value = value;
    }

    public Bar getValue() {
        return value;
    }

    public void setValue(Bar value) {
        this.value = value;
    }
}
