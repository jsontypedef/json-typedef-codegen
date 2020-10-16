package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Values {


  @JsonValue
  
  private Map<String, Value> value;


  
  public Values() {
  }
  



  public Map<String, Value> getValue() {
    return value;
  }


  public void setValue(Map<String, Value> value) {
    this.value = value;
  }

}
