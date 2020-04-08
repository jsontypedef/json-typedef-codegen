package com.jsontypedef.jtdcodegendemo;




public class Element {

  
  @JsonProperty("elementThing")
  private Object elementThing;


  
  public Element() {
  }
  


  public Object getElementThing() {
    return elementThing;
  }

  public void setElementThing(Object elementThing) {
    this.elementThing = elementThing;
  }

}
