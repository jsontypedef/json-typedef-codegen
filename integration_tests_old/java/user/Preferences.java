package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * Some preferences the user has indicated to us.
 */

public class Preferences {

  /**
   * User preferences around do-not-track
   */
  
  @JsonProperty("do_not_track")
  private PreferencesDoNotTrack doNotTrack;

  /**
   * A title we should use when addressing the user formally.
   */
  
  @JsonProperty("title")
  private PreferencesTitle title;


  
  public Preferences() {
  }
  


  /**
   * User preferences around do-not-track
   */
  public PreferencesDoNotTrack getDoNotTrack() {
    return doNotTrack;
  }

  /**
   * User preferences around do-not-track
   */
  public void setDoNotTrack(PreferencesDoNotTrack doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

  /**
   * A title we should use when addressing the user formally.
   */
  public PreferencesTitle getTitle() {
    return title;
  }

  /**
   * A title we should use when addressing the user formally.
   */
  public void setTitle(PreferencesTitle title) {
    this.title = title;
  }

}
