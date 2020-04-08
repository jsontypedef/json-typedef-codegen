package com.jsontypedef.jtdcodegendemo;




public class V1 extends Discriminator {

  
  @JsonProperty("user")
  private V1User user;


  
  public V1() {
  }
  


  public V1User getUser() {
    return user;
  }

  public void setUser(V1User user) {
    this.user = user;
  }

}
