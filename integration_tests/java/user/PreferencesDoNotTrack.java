package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

/**
 * User preferences around do-not-track
 */

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "version")
@JsonSubTypes({
  
    @JsonSubTypes.Type(name = "v0", value = PreferencesDoNotTrackV0.class),
  
    @JsonSubTypes.Type(name = "v1", value = PreferencesDoNotTrackV1.class),
  
})

public abstract class PreferencesDoNotTrack {


  


}
