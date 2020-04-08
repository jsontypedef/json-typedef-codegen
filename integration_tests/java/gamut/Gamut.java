package com.jsontypedef.jtdcodegendemo;




public class Gamut {

  
  @JsonProperty("elements")
  private Elements elements;

  
  @JsonProperty("enum")
  private Enum enum;

  
  @JsonProperty("discriminator")
  private Discriminator discriminator;

  
  @JsonProperty("empty")
  private Empty empty;

  
  @JsonProperty("type")
  private Type type;

  
  @JsonProperty("values")
  private Values values;


  
  public Gamut() {
  }
  


  public Elements getElements() {
    return elements;
  }

  public void setElements(Elements elements) {
    this.elements = elements;
  }

  public Enum getEnum() {
    return enum;
  }

  public void setEnum(Enum enum) {
    this.enum = enum;
  }

  public Discriminator getDiscriminator() {
    return discriminator;
  }

  public void setDiscriminator(Discriminator discriminator) {
    this.discriminator = discriminator;
  }

  public Empty getEmpty() {
    return empty;
  }

  public void setEmpty(Empty empty) {
    this.empty = empty;
  }

  public Type getType() {
    return type;
  }

  public void setType(Type type) {
    this.type = type;
  }

  public Values getValues() {
    return values;
  }

  public void setValues(Values values) {
    this.values = values;
  }

}
