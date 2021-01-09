package com.example;


import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**

 */

@JsonSerialize

public class Root {

    
    @JsonProperty("notnull_ref_notnull_string")
    private NotnullRefNotnullString notnullRefNotnullString;

    
    @JsonProperty("notnull_ref_null_string")
    private NotnullRefNullString notnullRefNullString;

    
    @JsonProperty("notnull_string")
    private NotnullString notnullString;

    
    @JsonProperty("null_ref_notnull_string")
    private NullRefNotnullString nullRefNotnullString;

    
    @JsonProperty("null_ref_null_string")
    private NullRefNullString nullRefNullString;

    
    @JsonProperty("null_string")
    private NullString nullString;


    public Root() {
    }


    /**

     */

    public NotnullRefNotnullString getNotnullRefNotnullString() {
        return this.notnullRefNotnullString;
    }

    /**

     */

    public void setNotnullRefNotnullString(NotnullRefNotnullString notnullRefNotnullString) {
        this.notnullRefNotnullString = notnullRefNotnullString;
    }

    /**

     */

    public NotnullRefNullString getNotnullRefNullString() {
        return this.notnullRefNullString;
    }

    /**

     */

    public void setNotnullRefNullString(NotnullRefNullString notnullRefNullString) {
        this.notnullRefNullString = notnullRefNullString;
    }

    /**

     */

    public NotnullString getNotnullString() {
        return this.notnullString;
    }

    /**

     */

    public void setNotnullString(NotnullString notnullString) {
        this.notnullString = notnullString;
    }

    /**

     */

    public NullRefNotnullString getNullRefNotnullString() {
        return this.nullRefNotnullString;
    }

    /**

     */

    public void setNullRefNotnullString(NullRefNotnullString nullRefNotnullString) {
        this.nullRefNotnullString = nullRefNotnullString;
    }

    /**

     */

    public NullRefNullString getNullRefNullString() {
        return this.nullRefNullString;
    }

    /**

     */

    public void setNullRefNullString(NullRefNullString nullRefNullString) {
        this.nullRefNullString = nullRefNullString;
    }

    /**

     */

    public NullString getNullString() {
        return this.nullString;
    }

    /**

     */

    public void setNullString(NullString nullString) {
        this.nullString = nullString;
    }

}
