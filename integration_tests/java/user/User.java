package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class User {

  
  @JsonProperty("id")
  private String id;

  
  @JsonProperty("labels")
  private Map<String, String> labels;

  
  @JsonProperty("name")
  private Name name;

  
  @JsonProperty("preferences")
  private Preferences preferences;

  
  @JsonProperty("first_known_location")
  private Location firstKnownLocation;

  
  @JsonProperty("last_known_location")
  private Location lastKnownLocation;


  
  public User() {
  }
  


  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public Map<String, String> getLabels() {
    return labels;
  }

  public void setLabels(Map<String, String> labels) {
    this.labels = labels;
  }

  public Name getName() {
    return name;
  }

  public void setName(Name name) {
    this.name = name;
  }

  public Preferences getPreferences() {
    return preferences;
  }

  public void setPreferences(Preferences preferences) {
    this.preferences = preferences;
  }

  public Location getFirstKnownLocation() {
    return firstKnownLocation;
  }

  public void setFirstKnownLocation(Location firstKnownLocation) {
    this.firstKnownLocation = firstKnownLocation;
  }

  public Location getLastKnownLocation() {
    return lastKnownLocation;
  }

  public void setLastKnownLocation(Location lastKnownLocation) {
    this.lastKnownLocation = lastKnownLocation;
  }

}
