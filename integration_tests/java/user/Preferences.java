package com.jsontypedef.jtdcodegendemo;




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
