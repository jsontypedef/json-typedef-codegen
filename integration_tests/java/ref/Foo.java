package com.jsontypedef.jtdcodegendemo;




public class Foo {

  @JsonValue
  
  private Bar value;


  
  public Foo() {
  }
  


  public Bar getValue() {
    return value;
  }

  public void setValue(Bar value) {
    this.value = value;
  }

}
