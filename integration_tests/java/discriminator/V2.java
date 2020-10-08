package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



public class V2 extends Discriminator {


  
  @JsonProperty("user")
  private V2User user;


  
  public V2() {
  }
  



  public V2User getUser() {
    return user;
  }


  public void setUser(V2User user) {
    this.user = user;
  }

}
