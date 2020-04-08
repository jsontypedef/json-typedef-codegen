package com.jsontypedef.jtdcodegendemo;




public class V2 extends Discriminator {

  
  @JsonProperty("user")
  private V2User user;


  
  public V2() {
  }
  


  public V2User getUser() {
    return user;
  }

  public void setUser(V2User user) {
    this.user = user;
  }

}
