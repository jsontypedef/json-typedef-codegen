package com.jsontypedef.jtdcodegendemo;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;

import java.util.List;
import java.util.Map;


public class Gamut {

  
  @JsonProperty("discriminator")
  private Discriminator discriminator;

  
  @JsonProperty("type")
  private Type type;

  
  @JsonProperty("empty")
  private Empty empty;

  
  @JsonProperty("enum")
  private Enum enum;

  
  @JsonProperty("elements")
  private Elements elements;

  
  @JsonProperty("values")
  private Values values;


  
  public Gamut() {
  }
  


  public Discriminator getDiscriminator() {
    return discriminator;
  }

  public void setDiscriminator(Discriminator discriminator) {
    this.discriminator = discriminator;
  }

  public Type getType() {
    return type;
  }

  public void setType(Type type) {
    this.type = type;
  }

  public Empty getEmpty() {
    return empty;
  }

  public void setEmpty(Empty empty) {
    this.empty = empty;
  }

  public Enum getEnum() {
    return enum;
  }

  public void setEnum(Enum enum) {
    this.enum = enum;
  }

  public Elements getElements() {
    return elements;
  }

  public void setElements(Elements elements) {
    this.elements = elements;
  }

  public Values getValues() {
    return values;
  }

  public void setValues(Values values) {
    this.values = values;
  }

}
