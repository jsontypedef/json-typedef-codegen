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


        /// <value>
        /// A multi-level do-not-track setting
        /// </value>
        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrackV1DoNotTrack DoNotTrack { get; set; }

        /// <value>
        /// Channels the user has opted out of tracking for.
        /// </value>
        [JsonProperty("opt_out_channels")]
        public IList<string> OptOutChannels { get; set; }





    }
}
