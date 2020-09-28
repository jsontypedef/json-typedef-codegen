package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Elements {

  @JsonValue
  
  private List<Element> value;


  
  public Elements() {
  }
  


  public List<Element> getValue() {
    return value;
  }

  public void setValue(List<Element> value) {
    this.value = value;
  }

}
