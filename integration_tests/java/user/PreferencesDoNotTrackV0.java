package com.jsontypedef.jtdcodegendemo;




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
