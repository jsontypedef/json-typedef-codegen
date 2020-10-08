package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * Our pre-GDPR do-not-track settings
 */

public class PreferencesDoNotTrackV0 extends PreferencesDoNotTrack {

  /**
   * An all-or-nothing do-not-track setting
   */
  
  @JsonProperty("do_not_track")
  private Boolean doNotTrack;


  
  public PreferencesDoNotTrackV0() {
  }
  


  /**
   * An all-or-nothing do-not-track setting
   */
  public Boolean getDoNotTrack() {
    return doNotTrack;
  }

  /**
   * An all-or-nothing do-not-track setting
   */
  public void setDoNotTrack(Boolean doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

}
