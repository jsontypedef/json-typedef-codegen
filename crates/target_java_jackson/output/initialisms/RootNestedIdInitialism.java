package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class RootNestedIdInitialism {
    @JsonProperty("json")
    private String json;

    @JsonProperty("normalword")
    private String normalword;

    public RootNestedIdInitialism() {
    }

    /**
     * Getter for json.<p>
     */
    public String getJson() {
        return json;
    }

    /**
     * Setter for json.<p>
     */
    public void setJson(String json) {
        this.json = json;
    }

    /**
     * Getter for normalword.<p>
     */
    public String getNormalword() {
        return normalword;
    }

    /**
     * Setter for normalword.<p>
     */
    public void setNormalword(String normalword) {
        this.normalword = normalword;
    }
}
