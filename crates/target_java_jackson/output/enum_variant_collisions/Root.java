package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum Root {
    @JsonProperty("FOO")
    FOO,

    @JsonProperty("Foo")
    FOO0,

    @JsonProperty("foo")
    FOO1,
}
