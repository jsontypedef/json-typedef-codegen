package com.jsontypedef.jtdcodegendemo;




public class NullableBoolean {

  @JsonValue
  
  private Boolean value;


  
  public NullableBoolean() {
  }
  


  public Boolean getValue() {
    return value;
  }

  public void setValue(Boolean value) {
    this.value = value;
  }

}
