package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



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
