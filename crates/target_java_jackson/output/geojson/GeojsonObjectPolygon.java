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

    /**
     * Getter for coordinates.<p>
     */
    public List<LinearRing> getCoordinates() {
        return coordinates;
    }

    /**
     * Setter for coordinates.<p>
     */
    public void setCoordinates(List<LinearRing> coordinates) {
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
