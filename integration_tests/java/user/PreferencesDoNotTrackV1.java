package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class PreferencesDoNotTrackV1 extends PreferencesDoNotTrack {

  
  @JsonProperty("opt_out_channels")
  private List<String> optOutChannels;

  
  @JsonProperty("do_not_track")
  private PreferencesDoNotTrackV1DoNotTrack doNotTrack;


  
  public PreferencesDoNotTrackV1() {
  }
  


  public List<String> getOptOutChannels() {
    return optOutChannels;
  }

  public void setOptOutChannels(List<String> optOutChannels) {
    this.optOutChannels = optOutChannels;
  }

  public PreferencesDoNotTrackV1DoNotTrack getDoNotTrack() {
    return doNotTrack;
  }

  public void setDoNotTrack(PreferencesDoNotTrackV1DoNotTrack doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

}
