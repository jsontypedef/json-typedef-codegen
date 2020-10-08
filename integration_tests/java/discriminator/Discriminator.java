package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;



@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "version")
@JsonSubTypes({
  
    @JsonSubTypes.Type(name = "v1", value = V1.class),
  
    @JsonSubTypes.Type(name = "v2", value = V2.class),
  
})

public abstract class Discriminator {


  


}
