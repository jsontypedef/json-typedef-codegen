package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


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
