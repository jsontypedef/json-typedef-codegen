package com.jsontypedef.jtdcodegendemo;




public class Gamut {

  
  @JsonProperty("empty")
  private Empty empty;

  
  @JsonProperty("type")
  private Type type;

  
  @JsonProperty("elements")
  private Elements elements;

  
  @JsonProperty("discriminator")
  private Discriminator discriminator;

  
  @JsonProperty("enum")
  private Enum enum;

  
  @JsonProperty("values")
  private Values values;


  
  public Gamut() {
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

  public Elements getElements() {
    return elements;
  }

  public void setElements(Elements elements) {
    this.elements = elements;
  }

  public Discriminator getDiscriminator() {
    return discriminator;
  }

  public void setDiscriminator(Discriminator discriminator) {
    this.discriminator = discriminator;
  }

  public Enum getEnum() {
    return enum;
  }

  public void setEnum(Enum enum) {
    this.enum = enum;
  }

  public Values getValues() {
    return values;
  }

  public void setValues(Values values) {
    this.values = values;
  }

}
