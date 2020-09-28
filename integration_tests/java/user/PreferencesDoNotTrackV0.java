package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class PreferencesDoNotTrackV0 extends PreferencesDoNotTrack {

  
  @JsonProperty("do_not_track")
  private Boolean doNotTrack;


  
  public PreferencesDoNotTrackV0() {
  }
  


  public Boolean getDoNotTrack() {
    return doNotTrack;
  }

  public void setDoNotTrack(Boolean doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

}
