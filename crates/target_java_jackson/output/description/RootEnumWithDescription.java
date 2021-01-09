package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * A description for enum
 */

public enum RootEnumWithDescription {

        /**
         * A description for X
         */

    @JsonProperty("X")
    X,

        /**
         * A description for Y
         */

    @JsonProperty("Y")
    Y,

        /**
         * A description for Z
         */

    @JsonProperty("Z")
    Z,

}
