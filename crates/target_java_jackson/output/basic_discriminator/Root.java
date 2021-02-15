package com.example;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
@JsonSubTypes({
    @JsonSubTypes.Type(name = "BAR_BAZ", value = RootBarBaz.class),
    @JsonSubTypes.Type(name = "QUUX", value = RootQuux.class),
})
public abstract class Root {
}
