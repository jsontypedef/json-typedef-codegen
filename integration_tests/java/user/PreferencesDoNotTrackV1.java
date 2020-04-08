package com.jsontypedef.jtdcodegendemo;




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
