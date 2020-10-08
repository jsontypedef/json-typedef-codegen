package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Float32 {


  @JsonValue
  
  private Float value;


  
  public Float32() {
  }
  



  public Float getValue() {
    return value;
  }


  public void setValue(Float value) {
    this.value = value;
  }

}
