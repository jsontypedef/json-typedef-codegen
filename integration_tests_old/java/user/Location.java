package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * A latitude / longitude pair indicating a position on Earth
 */

public class Location {

  /**
   * Latitude
   */
  
  @JsonProperty("lat")
  private String lat;

  /**
   * Longitude
   */
  
  @JsonProperty("lng")
  private String lng;


  
  public Location() {
  }
  


  /**
   * Latitude
   */
  public String getLat() {
    return lat;
  }

  /**
   * Latitude
   */
  public void setLat(String lat) {
    this.lat = lat;
  }

  /**
   * Longitude
   */
  public String getLng() {
    return lng;
  }

  /**
   * Longitude
   */
  public void setLng(String lng) {
    this.lng = lng;
  }

}
