package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Int32 {


  @JsonValue
  
  private Integer value;


  
  public Int32() {
  }
  



  public Integer getValue() {
    return value;
  }


  public void setValue(Integer value) {
    this.value = value;
  }

}
