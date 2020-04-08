package com.jsontypedef.jtdcodegendemo;




public class Timestamp {

  @JsonValue
  
  private OffsetDateTime value;


  
  public Timestamp() {
  }
  


  public OffsetDateTime getValue() {
    return value;
  }

  public void setValue(OffsetDateTime value) {
    this.value = value;
  }

}
