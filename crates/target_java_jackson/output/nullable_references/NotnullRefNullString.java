package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class NotnullRefNullString {
    @JsonValue
    private NullString value;

    public NotnullRefNullString() {
    }

    @JsonCreator
    public NotnullRefNullString(NullString value) {
        this.value = value;
    }

    public NullString getValue() {
        return value;
    }

    public void setValue(NullString value) {
        this.value = value;
    }
}
