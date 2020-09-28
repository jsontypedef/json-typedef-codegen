package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Int16 {

  @JsonValue
  
  private Short value;


  
  public Int16() {
  }
  


  public Short getValue() {
    return value;
  }

  public void setValue(Short value) {
    this.value = value;
  }

}
