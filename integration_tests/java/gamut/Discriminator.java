package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


  @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
  @JsonSubTypes({
    
      @JsonSubTypes.Type(name = "bar", value = DiscriminatorBar.class),
    
      @JsonSubTypes.Type(name = "baz", value = DiscriminatorBaz.class),
    
  })

public abstract class Discriminator {


  


}
