package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * Our post-GDPR do-not-track settings
 */

public class PreferencesDoNotTrackV1 extends PreferencesDoNotTrack {

  /**
   * A multi-level do-not-track setting
   */
  
  @JsonProperty("do_not_track")
  private PreferencesDoNotTrackV1DoNotTrack doNotTrack;

  /**
   * Channels the user has opted out of tracking for.
   */
  
  @JsonProperty("opt_out_channels")
  private List<String> optOutChannels;


  
  public PreferencesDoNotTrackV1() {
  }
  


  /**
   * A multi-level do-not-track setting
   */
  public PreferencesDoNotTrackV1DoNotTrack getDoNotTrack() {
    return doNotTrack;
  }

  /**
   * A multi-level do-not-track setting
   */
  public void setDoNotTrack(PreferencesDoNotTrackV1DoNotTrack doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

  /**
   * Channels the user has opted out of tracking for.
   */
  public List<String> getOptOutChannels() {
    return optOutChannels;
  }

  /**
   * Channels the user has opted out of tracking for.
   */
  public void setOptOutChannels(List<String> optOutChannels) {
    this.optOutChannels = optOutChannels;
  }

}
