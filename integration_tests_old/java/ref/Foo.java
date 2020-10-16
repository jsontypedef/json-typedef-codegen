package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class Foo {


  @JsonValue
  
  private Bar value;


  
  public Foo() {
  }
  



  public Bar getValue() {
    return value;
  }


  public void setValue(Bar value) {
    this.value = value;
  }

}
