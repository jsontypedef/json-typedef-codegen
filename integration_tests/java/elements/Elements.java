package com.jsontypedef.jtdcodegendemo;




public class Elements {

  @JsonValue
  
  private List<OffsetDateTime> value;


  
  public Elements() {
  }
  


  public List<OffsetDateTime> getValue() {
    return value;
  }

  public void setValue(List<OffsetDateTime> value) {
    this.value = value;
  }

}
