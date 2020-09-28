package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class D {

  
  @JsonProperty("a")
  private Integer a;


  
  public D() {
  }
  


  public Integer getA() {
    return a;
  }

  public void setA(Integer a) {
    this.a = a;
  }

}
