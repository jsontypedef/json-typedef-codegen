package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Properties {

  
  @JsonProperty("a")
  private String a;

  
  @JsonProperty("b")
  private OffsetDateTime b;

  
  @JsonProperty("d")
  private D d;

  
  @JsonProperty("c")
  private String c;


  
  public Properties() {
  }
  


  public String getA() {
    return a;
  }

  public void setA(String a) {
    this.a = a;
  }

  public OffsetDateTime getB() {
    return b;
  }

  public void setB(OffsetDateTime b) {
    this.b = b;
  }

  public D getD() {
    return d;
  }

  public void setD(D d) {
    this.d = d;
  }

  public String getC() {
    return c;
  }

  public void setC(String c) {
    this.c = c;
  }

}
