package com.example;


import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import com.fasterxml.jackson.annotation.JsonInclude;

import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.util.List;

/**
 * For type "MultiPoint", the "coordinates" member is an array of
 * positions.
 */

@JsonSerialize
@JsonIgnoreProperties(ignoreUnknown = true)
public class GeojsonObjectMultiPoint extends GeojsonObject {

    
    @JsonProperty("coordinates")
    private List<Position> coordinates;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bbox")
    private BoundingBox bbox;


    public GeojsonObjectMultiPoint() {
    }


    public List<Position> getCoordinates() {
        return this.coordinates;
    }

    public void setCoordinates(List<Position> coordinates) {
        this.coordinates = coordinates;
    }

    public BoundingBox getBbox() {
        return this.bbox;
    }

    public void setBbox(BoundingBox bbox) {
        this.bbox = bbox;
    }

}
