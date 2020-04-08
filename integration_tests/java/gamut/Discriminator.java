package com.jsontypedef.jtdcodegendemo;




  @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "foo")
  @JsonSubTypes({
    
      @JsonSubTypes.Type(DiscriminatorBar.class),
    
      @JsonSubTypes.Type(DiscriminatorBaz.class),
    
  })

public abstract class Discriminator {


  


}
