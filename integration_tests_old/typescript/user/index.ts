/**
 * A proper name.
 * 
 * Note that this is a string, and not some object with first/given name or a
 * last/family name. We have users across many cultures, and some of these
 * cultures use mononyms or otherwise don't map onto these concepts.
 */
export type Name = string;

/**
 * User preferences around do-not-track
 */
export type PreferencesDoNotTrack = PreferencesDoNotTrackV0 | PreferencesDoNotTrackV1;

/**
 * A latitude / longitude pair indicating a position on Earth
 */
export interface Location {
  /**
   * Latitude
   */
  lat: string;
  /**
   * Longitude
   */
  lng: string;
}

/**
 * Some preferences the user has indicated to us.
 */
export interface Preferences {
  /**
   * User preferences around do-not-track
   */
  doNotTrack: PreferencesDoNotTrack;
  /**
   * A title we should use when addressing the user formally.
   */
  title?: ("HRH" | "MR" | "MRS" | "MS" | "REV");
}

/**
 * Our pre-GDPR do-not-track settings
 */
export interface PreferencesDoNotTrackV0 {
  /**
   * An all-or-nothing do-not-track setting
   */
  doNotTrack: boolean;

  version: "v0";
}

/**
 * Our post-GDPR do-not-track settings
 */
export interface PreferencesDoNotTrackV1 {
  /**
   * A multi-level do-not-track setting
   */
  doNotTrack: ("ALL" | "ESSENTIAL_ONLY" | "NONE");
  /**
   * Channels the user has opted out of tracking for.
   */
  optOutChannels: string[];

  version: "v1";
}

/**
 * A user represents a person in our system.
 */
export interface User {
  /**
   * The first known location of this user
   */
  firstKnownLocation?: Location;
  /**
   * The ID of the user in our database.
   */
  id: string;
  /**
   * Free-form labels that we have put on the user.
   */
  labels: {[name: string]: string};
  /**
   * The last known location of this user
   */
  lastKnownLocation?: Location;
  /**
   * The user's name.
   */
  name: Name;
  /**
   * Some preferences the user has indicated to us.
   */
  preferences: Preferences;
}

