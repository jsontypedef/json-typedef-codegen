package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.time.OffsetDateTime;

@JsonSerialize
public class Root {
    @JsonProperty("foo")
    private OffsetDateTime foo;

    public Root() {
    }

    /**
     * Getter for foo.<p>
     */
    public OffsetDateTime getFoo() {
        return foo;
    }

    /**
     * Setter for foo.<p>
     */
    public void setFoo(OffsetDateTime foo) {
        this.foo = foo;
    }
}
