package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class DiscriminatorBar extends Discriminator {

  
  @JsonProperty("barThing")
  private Object barThing;


  
  public DiscriminatorBar() {
  }
  


  public Object getBarThing() {
    return barThing;
  }

  public void setBarThing(Object barThing) {
    this.barThing = barThing;
  }

}
