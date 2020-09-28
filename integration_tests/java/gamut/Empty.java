package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Empty {

  @JsonValue
  
  private Object value;


  
  public Empty() {
  }
  


  public Object getValue() {
    return value;
  }

  public void setValue(Object value) {
    this.value = value;
  }

}
