package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

/**

 */

public class Root {
    @JsonValue
    private UnsignedShort value;

    public Root() {
    }

    @JsonCreator
    public Root(UnsignedShort value) {
        this.value = value;
    }

    public UnsignedShort getValue() {
        return value;
    }

    public void setValue(UnsignedShort value) {
        this.value = value;
    }
}
