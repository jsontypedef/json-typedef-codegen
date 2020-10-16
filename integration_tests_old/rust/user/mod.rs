use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


/// A proper name.
/// 
/// Note that this is a string, and not some object with first/given name or a last/family name. We have users across many cultures, and some of these cultures use mononyms or otherwise don't map onto these concepts.
pub type Name = String;


/// User preferences around do-not-track
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum PreferencesDoNotTrack {

    /// Our pre-GDPR do-not-track settings
    #[serde(rename = "v0")]
    V0(PreferencesDoNotTrackV0),

    /// Our post-GDPR do-not-track settings
    #[serde(rename = "v1")]
    V1(PreferencesDoNotTrackV1),
}


/// A multi-level do-not-track setting
#[derive(Debug, Serialize, Deserialize)]
pub enum PreferencesDoNotTrackV1DoNotTrack {

    /// All forms of tracking permitted.
    #[serde(rename = "ALL")]
    All,

    /// Only essentialy forms of tracking permitted.
    #[serde(rename = "ESSENTIAL_ONLY")]
    EssentialOnly,

    /// No forms forms of tracking permitted.
    #[serde(rename = "NONE")]
    None,
}


/// A title we should use when addressing the user formally.
#[derive(Debug, Serialize, Deserialize)]
pub enum PreferencesTitle {

    /// Refer to this user as 'His/Her Royal Highness'
    #[serde(rename = "HRH")]
    Hrh,

    /// Refer to this user as 'Mr.'
    #[serde(rename = "MR")]
    Mr,

    /// Refer to this user as 'Mrs.'
    #[serde(rename = "MRS")]
    Mrs,

    /// Refer to this user as 'Ms.'
    #[serde(rename = "MS")]
    Ms,

    /// Refer to this user as 'Rev.'
    #[serde(rename = "REV")]
    Rev,
}


/// A latitude / longitude pair indicating a position on Earth
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {

    /// Latitude
    #[serde(rename = "lat")]
    lat: String,

    /// Longitude
    #[serde(rename = "lng")]
    lng: String,
}


/// Some preferences the user has indicated to us.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {

    /// User preferences around do-not-track
    #[serde(rename = "do_not_track")]
    do_not_track: PreferencesDoNotTrack,

    /// A title we should use when addressing the user formally.
    #[serde(rename = "title")]
    title: Option<PreferencesTitle>,
}


/// Our pre-GDPR do-not-track settings
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferencesDoNotTrackV0 {

    /// An all-or-nothing do-not-track setting
    #[serde(rename = "do_not_track")]
    do_not_track: bool,
}


/// Our post-GDPR do-not-track settings
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferencesDoNotTrackV1 {

    /// A multi-level do-not-track setting
    #[serde(rename = "do_not_track")]
    do_not_track: PreferencesDoNotTrackV1DoNotTrack,

    /// Channels the user has opted out of tracking for.
    #[serde(rename = "opt_out_channels")]
    opt_out_channels: Vec<String>,
}


/// A user represents a person in our system.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {

    /// The first known location of this user
    #[serde(rename = "first_known_location")]
    first_known_location: Option<Location>,

    /// The ID of the user in our database.
    #[serde(rename = "id")]
    id: String,

    /// Free-form labels that we have put on the user.
    #[serde(rename = "labels")]
    labels: HashMap<String, String>,

    /// The last known location of this user
    #[serde(rename = "last_known_location")]
    last_known_location: Option<Location>,

    /// The user's name.
    #[serde(rename = "name")]
    name: Name,

    /// Some preferences the user has indicated to us.
    #[serde(rename = "preferences")]
    preferences: Preferences,
}

