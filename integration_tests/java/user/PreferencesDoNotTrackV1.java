package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class PreferencesDoNotTrackV1 extends PreferencesDoNotTrack {

  
  @JsonProperty("do_not_track")
  private PreferencesDoNotTrackV1DoNotTrack doNotTrack;

  
  @JsonProperty("opt_out_channels")
  private List<String> optOutChannels;


  
  public PreferencesDoNotTrackV1() {
  }
  


  public PreferencesDoNotTrackV1DoNotTrack getDoNotTrack() {
    return doNotTrack;
  }

  public void setDoNotTrack(PreferencesDoNotTrackV1DoNotTrack doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

  public List<String> getOptOutChannels() {
    return optOutChannels;
  }

  public void setOptOutChannels(List<String> optOutChannels) {
    this.optOutChannels = optOutChannels;
  }

}
