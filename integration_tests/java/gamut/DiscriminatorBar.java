package com.jsontypedef.jtdcodegendemo;




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
