package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**
 * A description for discriminator variant
 */
@JsonSerialize
public class RootDiscriminatorWithDescriptionBar extends RootDiscriminatorWithDescription {
    public RootDiscriminatorWithDescriptionBar() {
    }
}
