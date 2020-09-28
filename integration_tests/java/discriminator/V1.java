package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class V1 extends Discriminator {

  
  @JsonProperty("user")
  private V1User user;


  
  public V1() {
  }
  


  public V1User getUser() {
    return user;
  }

  public void setUser(V1User user) {
    this.user = user;
  }

}
