package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum RootFooBar {
    @JsonProperty("x")
    X,

    @JsonProperty("y")
    Y,
}
