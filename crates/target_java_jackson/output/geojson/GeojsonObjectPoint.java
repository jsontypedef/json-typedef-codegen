package com.example;


import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import com.fasterxml.jackson.annotation.JsonInclude;

import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**
 * For type "Point", the "coordinates" member is a single position.
 */

@JsonSerialize
@JsonIgnoreProperties(ignoreUnknown = true)
public class GeojsonObjectPoint extends GeojsonObject {

    
    @JsonProperty("coordinates")
    private Position coordinates;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bbox")
    private BoundingBox bbox;


    public GeojsonObjectPoint() {
    }


    public Position getCoordinates() {
        return this.coordinates;
    }

    public void setCoordinates(Position coordinates) {
        this.coordinates = coordinates;
    }

    public BoundingBox getBbox() {
        return this.bbox;
    }

    public void setBbox(BoundingBox bbox) {
        this.bbox = bbox;
    }

}
