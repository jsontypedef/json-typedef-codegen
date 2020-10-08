using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// Our post-GDPR do-not-track settings
    /// </summary>
    
    public class PreferencesDoNotTrackV1 : PreferencesDoNotTrack
    {

        [JsonProperty("version")]
        public const string Version = "v1";


        /// <summary>
        /// A multi-level do-not-track setting
        /// </summary>
        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrackV1DoNotTrack DoNotTrack { get; set; }

        /// <summary>
        /// Channels the user has opted out of tracking for.
        /// </summary>
        [JsonProperty("opt_out_channels")]
        public IList<string> OptOutChannels { get; set; }





    }
}
