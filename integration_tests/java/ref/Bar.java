package com.jsontypedef.jtdcodegendemo;




public class Bar {

  @JsonValue
  
  private String value;


  
  public Bar() {
  }
  


  public String getValue() {
    return value;
  }

  public void setValue(String value) {
    this.value = value;
  }

}
