package com.jsontypedef.jtdcodegendemo;

/**
 * A title we should use when addressing the user formally.
 */
public enum PreferencesTitle {

  /**
   * Refer to this user as 'His/Her Royal Highness'
   */
  @JsonProperty("HRH")
  HRH,

  /**
   * Refer to this user as 'Mr.'
   */
  @JsonProperty("MR")
  MR,

  /**
   * Refer to this user as 'Mrs.'
   */
  @JsonProperty("MRS")
  MRS,

  /**
   * Refer to this user as 'Ms.'
   */
  @JsonProperty("MS")
  MS,

  /**
   * Refer to this user as 'Rev.'
   */
  @JsonProperty("REV")
  REV,

}
