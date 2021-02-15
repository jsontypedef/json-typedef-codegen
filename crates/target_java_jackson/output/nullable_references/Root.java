package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

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
     * Getter for notnullRefNotnullString.<p>
     */
    public NotnullRefNotnullString getNotnullRefNotnullString() {
        return notnullRefNotnullString;
    }

    /**
     * Setter for notnullRefNotnullString.<p>
     */
    public void setNotnullRefNotnullString(NotnullRefNotnullString notnullRefNotnullString) {
        this.notnullRefNotnullString = notnullRefNotnullString;
    }

    /**
     * Getter for notnullRefNullString.<p>
     */
    public NotnullRefNullString getNotnullRefNullString() {
        return notnullRefNullString;
    }

    /**
     * Setter for notnullRefNullString.<p>
     */
    public void setNotnullRefNullString(NotnullRefNullString notnullRefNullString) {
        this.notnullRefNullString = notnullRefNullString;
    }

    /**
     * Getter for notnullString.<p>
     */
    public NotnullString getNotnullString() {
        return notnullString;
    }

    /**
     * Setter for notnullString.<p>
     */
    public void setNotnullString(NotnullString notnullString) {
        this.notnullString = notnullString;
    }

    /**
     * Getter for nullRefNotnullString.<p>
     */
    public NullRefNotnullString getNullRefNotnullString() {
        return nullRefNotnullString;
    }

    /**
     * Setter for nullRefNotnullString.<p>
     */
    public void setNullRefNotnullString(NullRefNotnullString nullRefNotnullString) {
        this.nullRefNotnullString = nullRefNotnullString;
    }

    /**
     * Getter for nullRefNullString.<p>
     */
    public NullRefNullString getNullRefNullString() {
        return nullRefNullString;
    }

    /**
     * Setter for nullRefNullString.<p>
     */
    public void setNullRefNullString(NullRefNullString nullRefNullString) {
        this.nullRefNullString = nullRefNullString;
    }

    /**
     * Getter for nullString.<p>
     */
    public NullString getNullString() {
        return nullString;
    }

    /**
     * Setter for nullString.<p>
     */
    public void setNullString(NullString nullString) {
        this.nullString = nullString;
    }
}
