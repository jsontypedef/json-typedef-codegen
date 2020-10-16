package com.jsontypedef.jtdcodegendemo;

/**
 * A multi-level do-not-track setting
 */
public enum PreferencesDoNotTrackV1DoNotTrack {

  /**
   * All forms of tracking permitted.
   */
  @JsonProperty("ALL")
  ALL,

  /**
   * Only essentialy forms of tracking permitted.
   */
  @JsonProperty("ESSENTIAL_ONLY")
  ESSENTIAL_ONLY,

  /**
   * No forms forms of tracking permitted.
   */
  @JsonProperty("NONE")
  NONE,

}
