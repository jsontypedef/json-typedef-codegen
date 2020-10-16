package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Timestamp {


  @JsonValue
  
  private OffsetDateTime value;


  
  public Timestamp() {
  }
  



  public OffsetDateTime getValue() {
    return value;
  }


  public void setValue(OffsetDateTime value) {
    this.value = value;
  }

}
