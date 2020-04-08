package com.jsontypedef.jtdcodegendemo;




  @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "version")
  @JsonSubTypes({
    
      @JsonSubTypes.Type(PreferencesDoNotTrackV0.class),
    
      @JsonSubTypes.Type(PreferencesDoNotTrackV1.class),
    
  })

public abstract class PreferencesDoNotTrack {


  


}
