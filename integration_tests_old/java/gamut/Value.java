package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Value {


  
  @JsonProperty("valueThing")
  private Object valueThing;


  
  public Value() {
  }
  



  public Object getValueThing() {
    return valueThing;
  }


  public void setValueThing(Object valueThing) {
    this.valueThing = valueThing;
  }

}
