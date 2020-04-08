package com.jsontypedef.jtdcodegendemo;




public class Values {

  @JsonValue
  
  private Map<String, OffsetDateTime> value;


  
  public Values() {
  }
  


  public Map<String, OffsetDateTime> getValue() {
    return value;
  }

  public void setValue(Map<String, OffsetDateTime> value) {
    this.value = value;
  }

}
