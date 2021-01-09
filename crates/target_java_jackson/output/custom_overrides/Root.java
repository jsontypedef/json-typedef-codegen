package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("override_elements_container")
    private java.util.ArrayList<String> overrideElementsContainer;

    
    @JsonProperty("override_type_discriminator")
    private Object overrideTypeDiscriminator;

    
    @JsonProperty("override_type_enum")
    private RootOverrideTypeEnum overrideTypeEnum;

    
    @JsonProperty("override_type_expr")
    private Object overrideTypeExpr;

    
    @JsonProperty("override_type_properties")
    private Object overrideTypeProperties;

    
    @JsonProperty("override_values_container")
    private java.util.HashMap<String, String> overrideValuesContainer;


    public Root() {
    }


    /**

     */

    public java.util.ArrayList<String> getOverrideElementsContainer() {
        return this.overrideElementsContainer;
    }

    /**

     */

    public void setOverrideElementsContainer(java.util.ArrayList<String> overrideElementsContainer) {
        this.overrideElementsContainer = overrideElementsContainer;
    }

    /**

     */

    public Object getOverrideTypeDiscriminator() {
        return this.overrideTypeDiscriminator;
    }

    /**

     */

    public void setOverrideTypeDiscriminator(Object overrideTypeDiscriminator) {
        this.overrideTypeDiscriminator = overrideTypeDiscriminator;
    }

    /**

     */

    public RootOverrideTypeEnum getOverrideTypeEnum() {
        return this.overrideTypeEnum;
    }

    /**

     */

    public void setOverrideTypeEnum(RootOverrideTypeEnum overrideTypeEnum) {
        this.overrideTypeEnum = overrideTypeEnum;
    }

    /**

     */

    public Object getOverrideTypeExpr() {
        return this.overrideTypeExpr;
    }

    /**

     */

    public void setOverrideTypeExpr(Object overrideTypeExpr) {
        this.overrideTypeExpr = overrideTypeExpr;
    }

    /**

     */

    public Object getOverrideTypeProperties() {
        return this.overrideTypeProperties;
    }

    /**

     */

    public void setOverrideTypeProperties(Object overrideTypeProperties) {
        this.overrideTypeProperties = overrideTypeProperties;
    }

    /**

     */

    public java.util.HashMap<String, String> getOverrideValuesContainer() {
        return this.overrideValuesContainer;
    }

    /**

     */

    public void setOverrideValuesContainer(java.util.HashMap<String, String> overrideValuesContainer) {
        this.overrideValuesContainer = overrideValuesContainer;
    }

}
