package com.jsontypedef.jtdcodegendemo;




public class Elements {

  @JsonValue
  
  private List<Element> value;


  
  public Elements() {
  }
  


  public List<Element> getValue() {
    return value;
  }

  public void setValue(List<Element> value) {
    this.value = value;
  }

}
