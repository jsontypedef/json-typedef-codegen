package com.jsontypedef.jtdcodegendemo;




public class Properties {

  
  @JsonProperty("b")
  private OffsetDateTime b;

  
  @JsonProperty("a")
  private String a;

  
  @JsonProperty("c")
  private String c;

  
  @JsonProperty("d")
  private D d;


  
  public Properties() {
  }
  


  public OffsetDateTime getB() {
    return b;
  }

  public void setB(OffsetDateTime b) {
    this.b = b;
  }

  public String getA() {
    return a;
  }

  public void setA(String a) {
    this.a = a;
  }

  public String getC() {
    return c;
  }

  public void setC(String c) {
    this.c = c;
  }

  public D getD() {
    return d;
  }

  public void setD(D d) {
    this.d = d;
  }

}
