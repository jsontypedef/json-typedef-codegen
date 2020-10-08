package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class V2User {

  
  @JsonProperty("id")
  private String id;

  
  @JsonProperty("favoriteNumbers")
  private List<String> favoriteNumbers;


  
  public V2User() {
  }
  


  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public List<String> getFavoriteNumbers() {
    return favoriteNumbers;
  }

  public void setFavoriteNumbers(List<String> favoriteNumbers) {
    this.favoriteNumbers = favoriteNumbers;
  }

}
