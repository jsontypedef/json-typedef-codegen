package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Float64 {


  @JsonValue
  
  private Double value;


  
  public Float64() {
  }
  



  public Double getValue() {
    return value;
  }


  public void setValue(Double value) {
    this.value = value;
  }

}
