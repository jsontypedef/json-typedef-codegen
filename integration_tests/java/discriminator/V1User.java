package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class V1User {

  
  @JsonProperty("favoriteNumbers")
  private List<Integer> favoriteNumbers;

  
  @JsonProperty("id")
  private String id;


  
  public V1User() {
  }
  


  public List<Integer> getFavoriteNumbers() {
    return favoriteNumbers;
  }

  public void setFavoriteNumbers(List<Integer> favoriteNumbers) {
    this.favoriteNumbers = favoriteNumbers;
  }

  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

}
