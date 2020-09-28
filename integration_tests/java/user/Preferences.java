package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Preferences {

  
  @JsonProperty("do_not_track")
  private PreferencesDoNotTrack doNotTrack;

  
  @JsonProperty("title")
  private PreferencesTitle title;


  
  public Preferences() {
  }
  


  public PreferencesDoNotTrack getDoNotTrack() {
    return doNotTrack;
  }

  public void setDoNotTrack(PreferencesDoNotTrack doNotTrack) {
    this.doNotTrack = doNotTrack;
  }

  public PreferencesTitle getTitle() {
    return title;
  }

  public void setTitle(PreferencesTitle title) {
    this.title = title;
  }

}
