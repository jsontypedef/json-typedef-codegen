package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Uint16 {


  @JsonValue
  
  private Short value;


  
  public Uint16() {
  }
  



  public Short getValue() {
    return value;
  }


  public void setValue(Short value) {
    this.value = value;
  }

}
