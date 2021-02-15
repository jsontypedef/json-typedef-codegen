package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Root {
    @JsonProperty("http")
    private String http;

    @JsonProperty("id")
    private String id;

    @JsonProperty("nested_id_initialism")
    private RootNestedIdInitialism nestedIdInitialism;

    @JsonProperty("utf8")
    private String utf8;

    @JsonProperty("word_with_embedded_id_initialism")
    private String wordWithEmbeddedIdInitialism;

    @JsonProperty("word_with_trailing_initialism_id")
    private String wordWithTrailingInitialismId;

    public Root() {
    }

    /**
     * Getter for http.<p>
     */
    public String getHttp() {
        return http;
    }

    /**
     * Setter for http.<p>
     */
    public void setHttp(String http) {
        this.http = http;
    }

    /**
     * Getter for id.<p>
     */
    public String getId() {
        return id;
    }

    /**
     * Setter for id.<p>
     */
    public void setId(String id) {
        this.id = id;
    }

    /**
     * Getter for nestedIdInitialism.<p>
     */
    public RootNestedIdInitialism getNestedIdInitialism() {
        return nestedIdInitialism;
    }

    /**
     * Setter for nestedIdInitialism.<p>
     */
    public void setNestedIdInitialism(RootNestedIdInitialism nestedIdInitialism) {
        this.nestedIdInitialism = nestedIdInitialism;
    }

    /**
     * Getter for utf8.<p>
     */
    public String getUtf8() {
        return utf8;
    }

    /**
     * Setter for utf8.<p>
     */
    public void setUtf8(String utf8) {
        this.utf8 = utf8;
    }

    /**
     * Getter for wordWithEmbeddedIdInitialism.<p>
     */
    public String getWordWithEmbeddedIdInitialism() {
        return wordWithEmbeddedIdInitialism;
    }

    /**
     * Setter for wordWithEmbeddedIdInitialism.<p>
     */
    public void setWordWithEmbeddedIdInitialism(String wordWithEmbeddedIdInitialism) {
        this.wordWithEmbeddedIdInitialism = wordWithEmbeddedIdInitialism;
    }

    /**
     * Getter for wordWithTrailingInitialismId.<p>
     */
    public String getWordWithTrailingInitialismId() {
        return wordWithTrailingInitialismId;
    }

    /**
     * Setter for wordWithTrailingInitialismId.<p>
     */
    public void setWordWithTrailingInitialismId(String wordWithTrailingInitialismId) {
        this.wordWithTrailingInitialismId = wordWithTrailingInitialismId;
    }
}
