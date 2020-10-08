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
  
  private Map<String, OffsetDateTime> value;


  
  public Values() {
  }
  



  public Map<String, OffsetDateTime> getValue() {
    return value;
  }


  public void setValue(Map<String, OffsetDateTime> value) {
    this.value = value;
  }

}
