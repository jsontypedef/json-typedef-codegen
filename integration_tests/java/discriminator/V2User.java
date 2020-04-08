package com.jsontypedef.jtdcodegendemo;




public class V2User {

  
  @JsonProperty("favoriteNumbers")
  private List<String> favoriteNumbers;

  
  @JsonProperty("id")
  private String id;


  
  public V2User() {
  }
  


  public List<String> getFavoriteNumbers() {
    return favoriteNumbers;
  }

  public void setFavoriteNumbers(List<String> favoriteNumbers) {
    this.favoriteNumbers = favoriteNumbers;
  }

  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

}
