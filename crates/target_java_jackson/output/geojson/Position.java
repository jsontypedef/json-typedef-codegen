package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;

/**
 * A position is the fundamental geometry construct.
 * 
 * A position is an array of numbers.  There MUST be two or more elements.
 * The first two elements are longitude and latitude, or easting and
 * northing, precisely in that order and using decimal numbers.  Altitude
 * or elevation MAY be included as an optional third element.
 * 
 * Implementations SHOULD NOT extend positions beyond three elements
 * because the semantics of extra elements are unspecified and ambiguous.
 * Historically, some implementations have used a fourth element to carry a
 * linear referencing measure (sometimes denoted as "M") or a numerical
 * timestamp, but in most situations a parser will not be able to properly
 * interpret these values.  The interpretation and meaning of additional
 * elements is beyond the scope of this specification, and additional
 * elements MAY be ignored by parsers.
 * 
 * A line between two positions is a straight Cartesian line, the shortest
 * line between those two points in the coordinate reference system (see
 * Section 4).
 * 
 * In other words, every point on a line that does not cross the
 * antimeridian between a point (lon0, lat0) and (lon1, lat1) can be
 * calculated as
 * 
 * F(lon, lat) = (lon0 + (lon1 - lon0) * t, lat0 + (lat1 - lat0) * t)
 * 
 * with t being a real number greater than or equal to 0 and smaller than
 * or equal to 1.  Note that this line may markedly differ from the
 * geodesic path along the curved surface of the reference ellipsoid.
 * 
 * The same applies to the optional height element with the proviso that
 * the direction of the height is as specified in the coordinate reference
 * system.
 * 
 * Note that, again, this does not mean that a surface with equal height
 * follows, for example, the curvature of a body of water.  Nor is a
 * surface of equal height perpendicular to a plumb line.
 */

public class Position {
    @JsonValue
    private List<Double> value;

    public Position() {
    }

    @JsonCreator
    public Position(List<Double> value) {
        this.value = value;
    }

    public List<Double> getValue() {
        return value;
    }

    public void setValue(List<Double> value) {
        this.value = value;
    }
}
