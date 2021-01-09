package com.example;


import com.fasterxml.jackson.annotation.JsonSubTypes;

import com.fasterxml.jackson.annotation.JsonTypeInfo;

/**
 * A description for discriminator
 */

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
@JsonSubTypes({

    @JsonSubTypes.Type(name = "bar", value = RootDiscriminatorWithDescriptionBar.class),

})
public abstract class RootDiscriminatorWithDescription {
}
