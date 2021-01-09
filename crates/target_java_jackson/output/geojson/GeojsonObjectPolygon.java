package com.example;


import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import com.fasterxml.jackson.annotation.JsonInclude;

import com.fasterxml.jackson.annotation.JsonProperty;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.util.List;

/**
 * For type "Polygon", the "coordinates" member MUST be an array of
 * linear ring coordinate arrays.
 */

@JsonSerialize
@JsonIgnoreProperties(ignoreUnknown = true)
public class GeojsonObjectPolygon extends GeojsonObject {

    
    @JsonProperty("coordinates")
    private List<LinearRing> coordinates;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("bbox")
    private BoundingBox bbox;


    public GeojsonObjectPolygon() {
    }


    public List<LinearRing> getCoordinates() {
        return this.coordinates;
    }

    public void setCoordinates(List<LinearRing> coordinates) {
        this.coordinates = coordinates;
    }

    public BoundingBox getBbox() {
        return this.bbox;
    }

    public void setBbox(BoundingBox bbox) {
        this.bbox = bbox;
    }

}
