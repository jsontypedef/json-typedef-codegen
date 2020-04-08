package com.jsontypedef.jtdcodegendemo;




public class Value {

  
  @JsonProperty("valueThing")
  private Object valueThing;


  
  public Value() {
  }
  


  public Object getValueThing() {
    return valueThing;
  }

  public void setValueThing(Object valueThing) {
    this.valueThing = valueThing;
  }

}
