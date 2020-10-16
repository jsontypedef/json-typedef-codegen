package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class DiscriminatorBaz extends Discriminator {


  
  @JsonProperty("bazThing")
  private Object bazThing;


  
  public DiscriminatorBaz() {
  }
  



  public Object getBazThing() {
    return bazThing;
  }


  public void setBazThing(Object bazThing) {
    this.bazThing = bazThing;
  }

}
