package user


import "encoding/json"



// A proper name.
// 
// Note that this is a string, and not some object with first/given name or a last/family name. We have users across many cultures, and some of these cultures use mononyms or otherwise don't map onto these concepts.
type Name string

// A multi-level do-not-track setting
type PreferencesDoNotTrackV1DoNotTrack string

// User preferences around do-not-track
type PreferencesDoNotTrackVersion string

// A title we should use when addressing the user formally.
type PreferencesTitle string

// All forms of tracking permitted.
const PreferencesDoNotTrackV1DoNotTrackPreferencesDoNotTrackV1DoNotTrackALL PreferencesDoNotTrackV1DoNotTrack = "ALL"

// Only essentialy forms of tracking permitted.
const PreferencesDoNotTrackV1DoNotTrackPreferencesDoNotTrackV1DoNotTrackESSENTIALONLY PreferencesDoNotTrackV1DoNotTrack = "ESSENTIAL_ONLY"

// No forms forms of tracking permitted.
const PreferencesDoNotTrackV1DoNotTrackPreferencesDoNotTrackV1DoNotTrackNONE PreferencesDoNotTrackV1DoNotTrack = "NONE"

const PreferencesDoNotTrackVersionV0 PreferencesDoNotTrackVersion = "v0"

const PreferencesDoNotTrackVersionV1 PreferencesDoNotTrackVersion = "v1"

// Refer to this user as 'His/Her Royal Highness'
const PreferencesTitlePreferencesTitleHRH PreferencesTitle = "HRH"

// Refer to this user as 'Mr.'
const PreferencesTitlePreferencesTitleMR PreferencesTitle = "MR"

// Refer to this user as 'Mrs.'
const PreferencesTitlePreferencesTitleMRS PreferencesTitle = "MRS"

// Refer to this user as 'Ms.'
const PreferencesTitlePreferencesTitleMS PreferencesTitle = "MS"

// Refer to this user as 'Rev.'
const PreferencesTitlePreferencesTitleREV PreferencesTitle = "REV"


// A latitude / longitude pair indicating a position on Earth
type Location struct {

  // Latitude
  Lat string `json:"lat"`

  // Longitude
  Lng string `json:"lng"`
}


// Some preferences the user has indicated to us.
type Preferences struct {

  // User preferences around do-not-track
  DoNotTrack PreferencesDoNotTrack `json:"do_not_track"`

  // A title we should use when addressing the user formally.
  Title *PreferencesTitle `json:"title"`
}


// Our pre-GDPR do-not-track settings
type PreferencesDoNotTrackV0 struct {

  // An all-or-nothing do-not-track setting
  DoNotTrack bool `json:"do_not_track"`
}


// Our post-GDPR do-not-track settings
type PreferencesDoNotTrackV1 struct {

  // A multi-level do-not-track setting
  DoNotTrack PreferencesDoNotTrackV1DoNotTrack `json:"do_not_track"`

  // Channels the user has opted out of tracking for.
  OptOutChannels []string `json:"opt_out_channels"`
}


// A user represents a person in our system.
type User struct {

  // The first known location of this user
  FirstKnownLocation *Location `json:"first_known_location"`

  // The ID of the user in our database.
  Id string `json:"id"`

  // Free-form labels that we have put on the user.
  Labels map[string]string `json:"labels"`

  // The last known location of this user
  LastKnownLocation *Location `json:"last_known_location"`

  // The user's name.
  Name Name `json:"name"`

  // Some preferences the user has indicated to us.
  Preferences Preferences `json:"preferences"`
}


