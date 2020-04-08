package com.jsontypedef.jtdcodegendemo;




public class Ref {

  @JsonValue
  
  private Foo value;


  
  public Ref() {
  }
  


  public Foo getValue() {
    return value;
  }

  public void setValue(Foo value) {
    this.value = value;
  }

}
