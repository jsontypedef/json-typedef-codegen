package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * A proper name.
 * 
 * Note that this is a string, and not some object with first/given name or a
 * last/family name. We have users across many cultures, and some of these
 * cultures use mononyms or otherwise don't map onto these concepts.
 */

public class Name {


  @JsonValue
  
  private String value;


  
  public Name() {
  }
  



  public String getValue() {
    return value;
  }


  public void setValue(String value) {
    this.value = value;
  }

}
