package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class NullableBoolean {


  @JsonValue
  
  private Boolean value;


  
  public NullableBoolean() {
  }
  



  public Boolean getValue() {
    return value;
  }


  public void setValue(Boolean value) {
    this.value = value;
  }

}
