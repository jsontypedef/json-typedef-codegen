package com.jsontypedef.jtdcodegendemo;




public class Location {

  
  @JsonProperty("lat")
  private String lat;

  
  @JsonProperty("lng")
  private String lng;


  
  public Location() {
  }
  


  public String getLat() {
    return lat;
  }

  public void setLat(String lat) {
    this.lat = lat;
  }

  public String getLng() {
    return lng;
  }

  public void setLng(String lng) {
    this.lng = lng;
  }

}
