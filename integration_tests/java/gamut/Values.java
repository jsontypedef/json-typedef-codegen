package com.jsontypedef.jtdcodegendemo;




public class Values {

  @JsonValue
  
  private Map<String, Value> value;


  
  public Values() {
  }
  


  public Map<String, Value> getValue() {
    return value;
  }

  public void setValue(Map<String, Value> value) {
    this.value = value;
  }

}
