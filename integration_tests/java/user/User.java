package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * A user represents a person in our system.
 */

public class User {

  /**
   * The first known location of this user
   */
  
  @JsonProperty("first_known_location")
  private Location firstKnownLocation;

  /**
   * The ID of the user in our database.
   */
  
  @JsonProperty("id")
  private String id;

  /**
   * Free-form labels that we have put on the user.
   */
  
  @JsonProperty("labels")
  private Map<String, String> labels;

  /**
   * The last known location of this user
   */
  
  @JsonProperty("last_known_location")
  private Location lastKnownLocation;

  /**
   * The user's name.
   */
  
  @JsonProperty("name")
  private Name name;

  /**
   * Some preferences the user has indicated to us.
   */
  
  @JsonProperty("preferences")
  private Preferences preferences;


  
  public User() {
  }
  


  /**
   * The first known location of this user
   */
  public Location getFirstKnownLocation() {
    return firstKnownLocation;
  }

  /**
   * The first known location of this user
   */
  public void setFirstKnownLocation(Location firstKnownLocation) {
    this.firstKnownLocation = firstKnownLocation;
  }

  /**
   * The ID of the user in our database.
   */
  public String getId() {
    return id;
  }

  /**
   * The ID of the user in our database.
   */
  public void setId(String id) {
    this.id = id;
  }

  /**
   * Free-form labels that we have put on the user.
   */
  public Map<String, String> getLabels() {
    return labels;
  }

  /**
   * Free-form labels that we have put on the user.
   */
  public void setLabels(Map<String, String> labels) {
    this.labels = labels;
  }

  /**
   * The last known location of this user
   */
  public Location getLastKnownLocation() {
    return lastKnownLocation;
  }

  /**
   * The last known location of this user
   */
  public void setLastKnownLocation(Location lastKnownLocation) {
    this.lastKnownLocation = lastKnownLocation;
  }

  /**
   * The user's name.
   */
  public Name getName() {
    return name;
  }

  /**
   * The user's name.
   */
  public void setName(Name name) {
    this.name = name;
  }

  /**
   * Some preferences the user has indicated to us.
   */
  public Preferences getPreferences() {
    return preferences;
  }

  /**
   * Some preferences the user has indicated to us.
   */
  public void setPreferences(Preferences preferences) {
    this.preferences = preferences;
  }

}
