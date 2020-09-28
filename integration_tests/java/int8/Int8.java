package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Int8 {

  @JsonValue
  
  private Byte value;


  
  public Int8() {
  }
  


  public Byte getValue() {
    return value;
  }

  public void setValue(Byte value) {
    this.value = value;
  }

}
