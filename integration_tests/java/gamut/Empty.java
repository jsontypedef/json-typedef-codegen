package com.jsontypedef.jtdcodegendemo;




public class Empty {

  @JsonValue
  
  private Object value;


  
  public Empty() {
  }
  


  public Object getValue() {
    return value;
  }

  public void setValue(Object value) {
    this.value = value;
  }

}
