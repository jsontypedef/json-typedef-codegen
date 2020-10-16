package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class String {


  @JsonValue
  
  private String value;


  
  public String() {
  }
  



  public String getValue() {
    return value;
  }


  public void setValue(String value) {
    this.value = value;
  }

}
