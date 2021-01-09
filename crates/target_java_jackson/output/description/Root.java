package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("discriminator_with_description")
    private RootDiscriminatorWithDescription discriminatorWithDescription;

    
    @JsonProperty("enum_with_description")
    private RootEnumWithDescription enumWithDescription;

    
    @JsonProperty("long_description")
    private String longDescription;

    
    @JsonProperty("properties_with_description")
    private RootPropertiesWithDescription propertiesWithDescription;

    
    @JsonProperty("ref_with_description")
    private Baz refWithDescription;

    
    @JsonProperty("string_with_description")
    private String stringWithDescription;


    public Root() {
    }


    /**
     * A description for discriminator
     */

    public RootDiscriminatorWithDescription getDiscriminatorWithDescription() {
        return this.discriminatorWithDescription;
    }

    /**
     * A description for discriminator
     */

    public void setDiscriminatorWithDescription(RootDiscriminatorWithDescription discriminatorWithDescription) {
        this.discriminatorWithDescription = discriminatorWithDescription;
    }

    /**
     * A description for enum
     */

    public RootEnumWithDescription getEnumWithDescription() {
        return this.enumWithDescription;
    }

    /**
     * A description for enum
     */

    public void setEnumWithDescription(RootEnumWithDescription enumWithDescription) {
        this.enumWithDescription = enumWithDescription;
    }

    /**
     * Whereas disregard and contempt for human rights have resulted in
     * barbarous acts which have outraged the conscience of mankind, and the
     * advent of a world in which human beings shall enjoy freedom of speech and
     * belief and freedom from fear and want has been proclaimed as the highest
     * aspiration of the common people,
     */

    public String getLongDescription() {
        return this.longDescription;
    }

    /**
     * Whereas disregard and contempt for human rights have resulted in
     * barbarous acts which have outraged the conscience of mankind, and the
     * advent of a world in which human beings shall enjoy freedom of speech and
     * belief and freedom from fear and want has been proclaimed as the highest
     * aspiration of the common people,
     */

    public void setLongDescription(String longDescription) {
        this.longDescription = longDescription;
    }

    /**
     * A description for properties
     */

    public RootPropertiesWithDescription getPropertiesWithDescription() {
        return this.propertiesWithDescription;
    }

    /**
     * A description for properties
     */

    public void setPropertiesWithDescription(RootPropertiesWithDescription propertiesWithDescription) {
        this.propertiesWithDescription = propertiesWithDescription;
    }

    /**
     * A description for ref
     */

    public Baz getRefWithDescription() {
        return this.refWithDescription;
    }

    /**
     * A description for ref
     */

    public void setRefWithDescription(Baz refWithDescription) {
        this.refWithDescription = refWithDescription;
    }

    /**
     * A description for string
     */

    public String getStringWithDescription() {
        return this.stringWithDescription;
    }

    /**
     * A description for string
     */

    public void setStringWithDescription(String stringWithDescription) {
        this.stringWithDescription = stringWithDescription;
    }

}
