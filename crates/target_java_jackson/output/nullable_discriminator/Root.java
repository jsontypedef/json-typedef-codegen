package com.example;


import com.fasterxml.jackson.annotation.JsonSubTypes;

import com.fasterxml.jackson.annotation.JsonTypeInfo;

/**

 */

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
@JsonSubTypes({

    @JsonSubTypes.Type(name = "bar", value = RootBar.class),

    @JsonSubTypes.Type(name = "quux", value = RootQuux.class),

})
public abstract class Root {
}
