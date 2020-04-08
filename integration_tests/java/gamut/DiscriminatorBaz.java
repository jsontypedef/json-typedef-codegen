package com.jsontypedef.jtdcodegendemo;




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
