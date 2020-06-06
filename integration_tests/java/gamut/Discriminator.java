package com.jsontypedef.jtdcodegendemo;




  @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
  @JsonSubTypes({
    
      @JsonSubTypes.Type(DiscriminatorBaz.class),
    
      @JsonSubTypes.Type(DiscriminatorBar.class),
    
  })

public abstract class Discriminator {


  


}
