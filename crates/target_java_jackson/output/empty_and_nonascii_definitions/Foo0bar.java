// Code generated by jtd-codegen for Java + Jackson v0.2.1

package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Foo0bar {
    @JsonValue
    private String value;

    public Foo0bar() {
    }

    @JsonCreator
    public Foo0bar(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }
}
