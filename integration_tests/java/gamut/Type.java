package com.jsontypedef.jtdcodegendemo;




public class Type {

  
  @JsonProperty("int16")
  private Short int16;

  
  @JsonProperty("timestamp")
  private OffsetDateTime timestamp;

  
  @JsonProperty("string")
  private String string;

  
  @JsonProperty("float64")
  private Double float64;

  
  @JsonProperty("boolean")
  private Boolean boolean;

  
  @JsonProperty("uint8")
  private Byte uint8;

  
  @JsonProperty("float32")
  private Float float32;

  
  @JsonProperty("int32")
  private Integer int32;

  
  @JsonProperty("int8")
  private Byte int8;

  
  @JsonProperty("uint16")
  private Short uint16;

  
  @JsonProperty("uint32")
  private Integer uint32;


  
  public Type() {
  }
  


  public Short getInt16() {
    return int16;
  }

  public void setInt16(Short int16) {
    this.int16 = int16;
  }

  public OffsetDateTime getTimestamp() {
    return timestamp;
  }

  public void setTimestamp(OffsetDateTime timestamp) {
    this.timestamp = timestamp;
  }

  public String getString() {
    return string;
  }

  public void setString(String string) {
    this.string = string;
  }

  public Double getFloat64() {
    return float64;
  }

  public void setFloat64(Double float64) {
    this.float64 = float64;
  }

  public Boolean getBoolean() {
    return boolean;
  }

  public void setBoolean(Boolean boolean) {
    this.boolean = boolean;
  }

  public Byte getUint8() {
    return uint8;
  }

  public void setUint8(Byte uint8) {
    this.uint8 = uint8;
  }

  public Float getFloat32() {
    return float32;
  }

  public void setFloat32(Float float32) {
    this.float32 = float32;
  }

  public Integer getInt32() {
    return int32;
  }

  public void setInt32(Integer int32) {
    this.int32 = int32;
  }

  public Byte getInt8() {
    return int8;
  }

  public void setInt8(Byte int8) {
    this.int8 = int8;
  }

  public Short getUint16() {
    return uint16;
  }

  public void setUint16(Short uint16) {
    this.uint16 = uint16;
  }

  public Integer getUint32() {
    return uint32;
  }

  public void setUint32(Integer uint32) {
    this.uint32 = uint32;
  }

}
