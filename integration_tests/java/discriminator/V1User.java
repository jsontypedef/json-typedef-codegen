package com.jsontypedef.jtdcodegendemo;




public class V1User {

  
  @JsonProperty("id")
  private String id;

  
  @JsonProperty("favoriteNumbers")
  private List<Integer> favoriteNumbers;


  
  public V1User() {
  }
  


  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public List<Integer> getFavoriteNumbers() {
    return favoriteNumbers;
  }

  public void setFavoriteNumbers(List<Integer> favoriteNumbers) {
    this.favoriteNumbers = favoriteNumbers;
  }

}
