// Code generated by jtd-codegen for Java + Jackson v0.2.1

package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Root {
    @JsonValue
    private Integer value;

    public Root() {
    }

    @JsonCreator
    public Root(Integer value) {
        this.value = value;
    }

    public Integer getValue() {
        return value;
    }

    public void setValue(Integer value) {
        this.value = value;
    }
}
