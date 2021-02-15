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

    /**
     * Getter for coordinates.<p>
     */
    public Position getCoordinates() {
        return coordinates;
    }

    /**
     * Setter for coordinates.<p>
     */
    public void setCoordinates(Position coordinates) {
        this.coordinates = coordinates;
    }

    /**
     * Getter for bbox.<p>
     */
    public BoundingBox getBbox() {
        return bbox;
    }

    /**
     * Setter for bbox.<p>
     */
    public void setBbox(BoundingBox bbox) {
        this.bbox = bbox;
    }
}
