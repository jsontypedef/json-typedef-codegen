// Code generated by jtd-codegen for Java + Jackson v0.2.1

package com.example;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.util.List;

@JsonSerialize
public class Root {
    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bar")
    private List<String> bar;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("baz")
    private Boolean baz;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("foo")
    private String foo;

    public Root() {
    }

    /**
     * Getter for bar.<p>
     */
    public List<String> getBar() {
        return bar;
    }

    /**
     * Setter for bar.<p>
     */
    public void setBar(List<String> bar) {
        this.bar = bar;
    }

    /**
     * Getter for baz.<p>
     */
    public Boolean getBaz() {
        return baz;
    }

    /**
     * Setter for baz.<p>
     */
    public void setBaz(Boolean baz) {
        this.baz = baz;
    }

    /**
     * Getter for foo.<p>
     */
    public String getFoo() {
        return foo;
    }

    /**
     * Setter for foo.<p>
     */
    public void setFoo(String foo) {
        this.foo = foo;
    }
}
