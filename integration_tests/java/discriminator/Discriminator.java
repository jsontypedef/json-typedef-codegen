package com.jsontypedef.jtdcodegendemo;




  @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "version")
  @JsonSubTypes({
    
      @JsonSubTypes.Type(V1.class),
    
      @JsonSubTypes.Type(V2.class),
    
  })

public abstract class Discriminator {


  


}
