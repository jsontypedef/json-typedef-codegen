package com.jsontypedef.jtdcodegendemo;




public class Name {

  @JsonValue
  
  private String value;


  
  public Name() {
  }
  


  public String getValue() {
    return value;
  }

  public void setValue(String value) {
    this.value = value;
  }

}
