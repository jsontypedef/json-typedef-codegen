package com.jsontypedef.jtdcodegendemo;




public class User {

  
  @JsonProperty("preferences")
  private Preferences preferences;

  
  @JsonProperty("id")
  private String id;

  
  @JsonProperty("labels")
  private Map<String, String> labels;

  
  @JsonProperty("name")
  private Name name;

  
  @JsonProperty("first_known_location")
  private Location firstKnownLocation;

  
  @JsonProperty("last_known_location")
  private Location lastKnownLocation;


  
  public User() {
  }
  


  public Preferences getPreferences() {
    return preferences;
  }

  public void setPreferences(Preferences preferences) {
    this.preferences = preferences;
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
