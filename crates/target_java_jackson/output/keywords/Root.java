package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("for")
    private For for_;

    
    @JsonProperty("object")
    private Object object;


    public Root() {
    }


    /**

     */

    public For getFor_() {
        return this.for_;
    }

    /**

     */

    public void setFor_(For for_) {
        this.for_ = for_;
    }

    /**

     */

    public Object getObject() {
        return this.object;
    }

    /**

     */

    public void setObject(Object object) {
        this.object = object;
    }

}
