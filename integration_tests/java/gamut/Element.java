package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Element {

  
  @JsonProperty("elementThing")
  private Object elementThing;


  
  public Element() {
  }
  


  public Object getElementThing() {
    return elementThing;
  }

  public void setElementThing(Object elementThing) {
    this.elementThing = elementThing;
  }

}
